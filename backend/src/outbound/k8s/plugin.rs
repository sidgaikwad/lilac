use std::sync::Arc;

use async_trait::async_trait;
use k8s_openapi::{
    api::{batch::v1::Job, core::v1::Node},
    apimachinery::pkg::api::resource::Quantity,
};
use kube::api::{Api, DeleteParams, ListParams, PostParams};
use serde_json::json;
use uuid::Uuid;

use crate::domain::{
    cluster::ports::ClusterRepository,
    scheduler::{
        models::{ClusterSnapshot, GpuInfo, NodeSnapshot},
        ports::{ComputePlatform, PlatformCapabilities, SchedulingDecision},
    },
    training_job::{
        models::{ResourceRequirements, TrainingJob},
        ports::TrainingJobRepository,
    },
};

use super::factory::KubeClientFactory;

pub struct KubernetesPlugin {
    kube_client_factory: Arc<dyn KubeClientFactory>,
    cluster_repo: Arc<dyn ClusterRepository>,
    job_repo: Arc<dyn TrainingJobRepository>,
}

impl KubernetesPlugin {
    pub fn new(
        kube_client_factory: Arc<dyn KubeClientFactory>,
        cluster_repo: Arc<dyn ClusterRepository>,
        job_repo: Arc<dyn TrainingJobRepository>,
    ) -> Self {
        Self {
            kube_client_factory,
            cluster_repo,
            job_repo,
        }
    }

    async fn get_cluster_snapshot(
        &self,
        cluster_id: &Uuid,
    ) -> Result<ClusterSnapshot, anyhow::Error> {
        let cluster = self
            .cluster_repo
            .get_cluster_by_id(&(*cluster_id).into())
            .await?;
        let client = self.kube_client_factory.create_client(&cluster).await?;

        let nodes_api: Api<Node> = Api::all(client);
        let nodes = nodes_api.list(&ListParams::default()).await?;

        let mut node_snapshots = Vec::new();

        for node in nodes {
            if let Some(snapshot) = self.node_to_snapshot(node) {
                node_snapshots.push(snapshot);
            }
        }

        Ok(ClusterSnapshot {
            cluster_id: *cluster_id,
            nodes: node_snapshots,
        })
    }
}

#[async_trait]
impl ComputePlatform for KubernetesPlugin {
    fn platform_type(&self) -> &'static str {
        "kubernetes"
    }

    fn capabilities(&self) -> PlatformCapabilities {
        PlatformCapabilities {
            supports_gpus: true,
        }
    }

    async fn find_suitable_node(
        &self,
        cluster_id: &Uuid,
        requirements: &ResourceRequirements,
    ) -> Result<Option<SchedulingDecision>, anyhow::Error> {
        let snapshot = self.get_cluster_snapshot(cluster_id).await?;

        for node in snapshot.nodes {
            tracing::debug!(
                target: "scheduler",
                node = %node.name,
                "Checking node for suitability."
            );

            if node.available_cpu_millicores < requirements.cpu_millicores {
                tracing::debug!(
                    target: "scheduler",
                    node = %node.name,
                    reason = "insufficient_cpu",
                    required = requirements.cpu_millicores,
                    available = node.available_cpu_millicores,
                    "Node rejected."
                );
                continue;
            }

            if node.available_memory_mb < requirements.memory_mb {
                tracing::debug!(
                    target: "scheduler",
                    node = %node.name,
                    reason = "insufficient_memory",
                    required = requirements.memory_mb,
                    available = node.available_memory_mb,
                    "Node rejected."
                );
                continue;
            }

            let gpu_match = match &requirements.gpus {
                None => true,
                Some(gpu_req) => {
                    let suitable_gpus: Vec<_> = node
                        .gpus
                        .iter()
                        .filter(|node_gpu| {
                            let model_match = gpu_req
                                .model
                                .as_ref()
                                .map_or(true, |req_model| &node_gpu.model == req_model);
                            let memory_match = gpu_req
                                .memory_gb
                                .map_or(true, |req_mem| node_gpu.memory_gb >= req_mem);
                            model_match && memory_match
                        })
                        .collect();
                    
                    let has_enough_gpus = suitable_gpus.len() >= gpu_req.count as usize;
                    if !has_enough_gpus {
                        tracing::debug!(
                            target: "scheduler",
                            node = %node.name,
                            reason = "insufficient_gpus",
                            required_count = gpu_req.count,
                            available_count = suitable_gpus.len(),
                            required_model = ?gpu_req.model,
                            "Node rejected."
                        );
                    }
                    has_enough_gpus
                }
            };

            if !gpu_match {
                continue;
            }

            tracing::info!(
                target: "scheduler",
                node = %node.name,
                cluster_id = %cluster_id,
                "Found suitable node for job."
            );

            return Ok(Some(SchedulingDecision {
                cluster_id: *cluster_id,
                node_name: node.name.clone(),
            }));
        }

        Ok(None)
    }

    async fn allocate_job(
        &self,
        job: &TrainingJob,
        decision: &SchedulingDecision,
    ) -> Result<(), anyhow::Error> {
        let cluster = self
            .cluster_repo
            .get_cluster_by_id(&decision.cluster_id.into())
            .await?;
        let client = self.kube_client_factory.create_client(&cluster).await?;

        let namespace = client.default_namespace();
        let job_name = format!("training-job-{}", job.id);

        let jobs: Api<Job> = Api::namespaced(client.clone(), namespace);

        let mut limits = serde_json::Map::new();
        limits.insert(
            "cpu".to_string(),
            json!(format!("{}m", job.resource_requirements.cpu_millicores)),
        );
        limits.insert(
            "memory".to_string(),
            json!(format!("{}Mi", job.resource_requirements.memory_mb)),
        );
        let mut node_selector = serde_json::Map::new();
        if let Some(gpu_req) = &job.resource_requirements.gpus {
            limits.insert("nvidia.com/gpu".to_string(), json!(gpu_req.count.to_string()));
            if let Some(model) = &gpu_req.model {
                node_selector.insert("nvidia.com/gpu.product".to_string(), json!(model));
            }
        }

        let job_manifest = json!({
            "apiVersion": "batch/v1",
            "kind": "Job",
            "metadata": {
                "name": job_name.clone(),
                "namespace": namespace,
            },
            "spec": {
                "template": {
                    "spec": {
                        "nodeName": decision.node_name,
                        "nodeSelector": node_selector,
                        "containers": [{
                            "name": "training-container",
                            "image": job.definition,
                            "env": [
                                { "name": "LILAC_JOB_ID", "value": job.id.to_string() },
                                // TODO: Make API server address configurable
                                { "name": "LILAC_API_ENDPOINT", "value": "http://lilac-api-service:8080" },
                                // TODO: Inject a short-lived auth token
                                { "name": "LILAC_AUTH_TOKEN", "value": "" }
                            ],
                            "resources": {
                                "requests": {
                                    "cpu": format!("{}m", job.resource_requirements.cpu_millicores),
                                    "memory": format!("{}Mi", job.resource_requirements.memory_mb)
                                },
                                "limits": limits
                            }
                        }],
                        "restartPolicy": "OnFailure"
                    }
                },
                "backoffLimit": 4
            }
        });

        let job_resource: Job = serde_json::from_value(job_manifest)?;
        jobs.create(&PostParams::default(), &job_resource)
            .await?;

        Ok(())
    }

    async fn deallocate_job(&self, job_id: &Uuid) -> Result<(), anyhow::Error> {
        let job = self
            .job_repo
            .get_training_jobs(crate::domain::training_job::models::GetTrainingJobsFilters {
                id: Some(*job_id),
                ..Default::default()
            })
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("Job not found"))?;

        let cluster_id = job
            .scheduled_cluster_id
            .ok_or_else(|| anyhow::anyhow!("Job has not been scheduled on any cluster yet"))?;

        let cluster = self
            .cluster_repo
            .get_cluster_by_id(&cluster_id.into())
            .await?;
        let client = self.kube_client_factory.create_client(&cluster).await?;

        let namespace = client.default_namespace();
        let job_name = format!("training-job-{}", job_id);
        let jobs: Api<Job> = Api::namespaced(client.clone(), namespace);

        jobs.delete(&job_name, &DeleteParams::default()).await?;

        Ok(())
    }
}

impl KubernetesPlugin {
    fn node_to_snapshot(&self, node: Node) -> Option<NodeSnapshot> {
        let name = node.metadata.name?;
        let status = node.status?;
        let allocatable = status.allocatable?;

        let available_cpu_millicores =
            self.parse_quantity(allocatable.get("cpu")?).map_or(0, |q| q as i32);
        let available_memory_mb = self
            .parse_quantity(allocatable.get("memory")?)
            .map_or(0, |q| (q / (1024 * 1024)) as i32);

        let gpus = self.parse_gpu_info(&allocatable, &node.metadata.labels);

        Some(NodeSnapshot {
            name,
            available_cpu_millicores,
            available_memory_mb,
            gpus,
        })
    }

    fn parse_quantity(&self, quantity: &Quantity) -> Option<u64> {
        // This is a simplified parser. It handles common suffixes.
        // For production, a more robust library might be needed.
        let quantity_str = &quantity.0;
        if let Ok(val) = quantity_str.parse::<u64>() {
            return Some(val);
        }
        if let Some(val_str) = quantity_str.strip_suffix('m') {
            return val_str.parse::<u64>().ok(); // millicores are returned as is
        }
        let (num_str, suffix) = quantity_str.split_at(quantity_str.len() - 2);
        let num = num_str.parse::<u64>().ok()?;
        match suffix {
            "Ki" => Some(num * 1024),
            "Mi" => Some(num * 1024_u64.pow(2)),
            "Gi" => Some(num * 1024_u64.pow(3)),
            "Ti" => Some(num * 1024_u64.pow(4)),
            _ => None,
        }
    }

    fn parse_gpu_info(
        &self,
        allocatable: &std::collections::BTreeMap<String, Quantity>,
        labels: &Option<std::collections::BTreeMap<String, String>>,
    ) -> Vec<GpuInfo> {
        let gpu_count = allocatable
            .get("nvidia.com/gpu")
            .and_then(|q| q.0.parse::<i32>().ok())
            .unwrap_or(0);

        if gpu_count == 0 {
            return vec![];
        }

        let model = labels
            .as_ref()
            .and_then(|l| {
                l.get("nvidia.com/gpu.product")
                    .or_else(|| l.get("nvidia.com/gpu-product"))
            })
            .cloned()
            .unwrap_or_else(|| "Unknown NVIDIA GPU".to_string());

        // This is a simplification. We assume all GPUs on the node are the same model
        // and we don't have a reliable way to get memory per GPU from standard node metrics.
        // We'll return a single GpuInfo entry representing the pool.
        vec![GpuInfo {
            model,
            memory_gb: 0, // Placeholder, as this is not easily discovered.
        }; gpu_count as usize]
    }
}