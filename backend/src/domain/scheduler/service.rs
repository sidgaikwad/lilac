use std::sync::Arc;

use tracing::{error, info, warn};

use super::{
    manager::ComputePlatformManager,
    models::{ClusterSnapshot, NodeSnapshot},
};
use crate::domain::training_job::{
    models::{ResourceRequirements, TrainingJobStatus},
    ports::TrainingJobRepository,
};

pub struct SchedulerService {
    platform_manager: Arc<ComputePlatformManager>,
    job_repo: Arc<dyn TrainingJobRepository>,
}

impl SchedulerService {
    pub fn new(
        platform_manager: Arc<ComputePlatformManager>,
        job_repo: Arc<dyn TrainingJobRepository>,
    ) -> Self {
        Self {
            platform_manager,
            job_repo,
        }
    }

    pub async fn run_cycle(&self) {
        info!("Starting scheduler cycle");

        let queued_jobs = match self.job_repo.get_queued_jobs_with_targets().await {
            Ok(jobs) => jobs,
            Err(e) => {
                error!("Failed to fetch queued jobs: {}", e);
                return;
            }
        };

        info!("Found {} queued jobs", queued_jobs.len());

        for job_with_targets in queued_jobs {
            let job = job_with_targets.job;
            info!("Processing job {}", job.id);

            for target in job_with_targets.targets {
                info!(
                    "Attempting to schedule job {} on cluster {}",
                    job.id, target.cluster_id
                );

                let plugin = match self
                    .platform_manager
                    .get_platform_for_cluster(&target.cluster_id)
                    .await
                {
                    Ok(p) => p,
                    Err(e) => {
                        warn!(
                            "Failed to get platform for cluster {}: {}. Skipping target.",
                            target.cluster_id, e
                        );
                        continue;
                    }
                };

                let snapshot = match plugin.get_cluster_snapshot(&target.cluster_id).await {
                    Ok(s) => s,
                    Err(e) => {
                        warn!(
                            "Failed to get snapshot for cluster {}: {}. Skipping target.",
                            target.cluster_id, e
                        );
                        continue;
                    }
                };

                if let Some(node) = find_suitable_node(&snapshot, &job.resource_requirements) {
                    info!(
                        "Found suitable node {} for job {} on cluster {}",
                        node.name, job.id, target.cluster_id
                    );

                    match plugin
                        .allocate_job(&job, &target.cluster_id, &node.name)
                        .await
                    {
                        Ok(_) => {
                            info!("Successfully allocated job {} to node {}", job.id, node.name);
                            if let Err(e) = self
                                .job_repo
                                .mark_as_starting(job.id, target.cluster_id)
                                .await
                            {
                                error!("Failed to update job {} status: {}", job.id, e);
                            }
                            // Break from the inner loop to move to the next job
                            break;
                        }
                        Err(e) => {
                            error!(
                                "Failed to allocate job {} to node {}: {}",
                                job.id, node.name, e
                            );
                        }
                    }
                } else {
                    info!(
                        "No suitable node found for job {} on cluster {}",
                        job.id, target.cluster_id
                    );
                }
            }
        }
        info!("Scheduler cycle finished");
    }
}

fn find_suitable_node<'a>(
    snapshot: &'a ClusterSnapshot,
    requirements: &ResourceRequirements,
) -> Option<&'a NodeSnapshot> {
    snapshot.nodes.iter().find(|node| {
        // First, check basic CPU and memory requirements
        if node.available_cpu_millicores < requirements.cpu_millicores
            || node.available_memory_mb < requirements.memory_mb
        {
            return false;
        }

        // Next, check GPU requirements
        match &requirements.gpus {
            // If the job doesn't require GPUs, the node is suitable
            None => true,

            // If the job requires GPUs, check if the node can satisfy them
            Some(gpu_req) => {
                // Filter the node's available GPUs based on the job's requirements
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

                // Check if the number of suitable GPUs is sufficient
                suitable_gpus.len() >= gpu_req.count as usize
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        scheduler::models::{ClusterSnapshot, GpuInfo, NodeSnapshot},
        training_job::models::{GpuRequirement, ResourceRequirements},
    };
    use uuid::Uuid;

    fn basic_node_snapshot(gpus: Vec<GpuInfo>) -> NodeSnapshot {
        NodeSnapshot {
            name: "test-node".to_string(),
            available_cpu_millicores: 4000,
            available_memory_mb: 8000,
            gpus,
        }
    }

    fn basic_cluster_snapshot(nodes: Vec<NodeSnapshot>) -> ClusterSnapshot {
        ClusterSnapshot {
            cluster_id: Uuid::new_v4(),
            nodes,
        }
    }

    #[test]
    fn test_find_node_no_gpu_req() {
        let snapshot = basic_cluster_snapshot(vec![basic_node_snapshot(vec![])]);
        let reqs = ResourceRequirements {
            cpu_millicores: 1000,
            memory_mb: 2000,
            gpus: None,
        };
        assert!(find_suitable_node(&snapshot, &reqs).is_some());
    }

    #[test]
    fn test_find_node_simple_gpu_req_success() {
        let node = basic_node_snapshot(vec![GpuInfo {
            model: "A100".to_string(),
            memory_gb: 40,
        }]);
        let snapshot = basic_cluster_snapshot(vec![node]);
        let reqs = ResourceRequirements {
            cpu_millicores: 1000,
            memory_mb: 2000,
            gpus: Some(GpuRequirement {
                count: 1,
                model: None,
                memory_gb: None,
            }),
        };
        assert!(find_suitable_node(&snapshot, &reqs).is_some());
    }

    #[test]
    fn test_find_node_specific_gpu_model_success() {
        let node = basic_node_snapshot(vec![GpuInfo {
            model: "A100".to_string(),
            memory_gb: 40,
        }]);
        let snapshot = basic_cluster_snapshot(vec![node]);
        let reqs = ResourceRequirements {
            cpu_millicores: 1000,
            memory_mb: 2000,
            gpus: Some(GpuRequirement {
                count: 1,
                model: Some("A100".to_string()),
                memory_gb: None,
            }),
        };
        assert!(find_suitable_node(&snapshot, &reqs).is_some());
    }

    #[test]
    fn test_find_node_specific_gpu_model_failure() {
        let node = basic_node_snapshot(vec![GpuInfo {
            model: "V100".to_string(),
            memory_gb: 32,
        }]);
        let snapshot = basic_cluster_snapshot(vec![node]);
        let reqs = ResourceRequirements {
            cpu_millicores: 1000,
            memory_mb: 2000,
            gpus: Some(GpuRequirement {
                count: 1,
                model: Some("A100".to_string()),
                memory_gb: None,
            }),
        };
        assert!(find_suitable_node(&snapshot, &reqs).is_none());
    }

    #[test]
    fn test_find_node_gpu_memory_success() {
        let node = basic_node_snapshot(vec![GpuInfo {
            model: "A100".to_string(),
            memory_gb: 40,
        }]);
        let snapshot = basic_cluster_snapshot(vec![node]);
        let reqs = ResourceRequirements {
            cpu_millicores: 1000,
            memory_mb: 2000,
            gpus: Some(GpuRequirement {
                count: 1,
                model: None,
                memory_gb: Some(40),
            }),
        };
        assert!(find_suitable_node(&snapshot, &reqs).is_some());
    }

    #[test]
    fn test_find_node_gpu_memory_failure() {
        let node = basic_node_snapshot(vec![GpuInfo {
            model: "A100".to_string(),
            memory_gb: 20,
        }]);
        let snapshot = basic_cluster_snapshot(vec![node]);
        let reqs = ResourceRequirements {
            cpu_millicores: 1000,
            memory_mb: 2000,
            gpus: Some(GpuRequirement {
                count: 1,
                model: None,
                memory_gb: Some(40),
            }),
        };
        assert!(find_suitable_node(&snapshot, &reqs).is_none());
    }

    #[test]
    fn test_find_node_multiple_gpus_success() {
        let node = basic_node_snapshot(vec![
            GpuInfo {
                model: "A100".to_string(),
                memory_gb: 40,
            },
            GpuInfo {
                model: "A100".to_string(),
                memory_gb: 40,
            },
        ]);
        let snapshot = basic_cluster_snapshot(vec![node]);
        let reqs = ResourceRequirements {
            cpu_millicores: 1000,
            memory_mb: 2000,
            gpus: Some(GpuRequirement {
                count: 2,
                model: Some("A100".to_string()),
                memory_gb: None,
            }),
        };
        assert!(find_suitable_node(&snapshot, &reqs).is_some());
    }

    #[test]
    fn test_find_node_multiple_gpus_failure_not_enough_gpus() {
        let node = basic_node_snapshot(vec![GpuInfo {
            model: "A100".to_string(),
            memory_gb: 40,
        }]);
        let snapshot = basic_cluster_snapshot(vec![node]);
        let reqs = ResourceRequirements {
            cpu_millicores: 1000,
            memory_mb: 2000,
            gpus: Some(GpuRequirement {
                count: 2,
                model: Some("A100".to_string()),
                memory_gb: None,
            }),
        };
        assert!(find_suitable_node(&snapshot, &reqs).is_none());
    }

    #[test]
    fn test_find_node_cpu_failure() {
        let snapshot = basic_cluster_snapshot(vec![basic_node_snapshot(vec![])]);
        let reqs = ResourceRequirements {
            cpu_millicores: 5000,
            memory_mb: 2000,
            gpus: None,
        };
        assert!(find_suitable_node(&snapshot, &reqs).is_none());
    }

    #[test]
    fn test_find_node_memory_failure() {
        let snapshot = basic_cluster_snapshot(vec![basic_node_snapshot(vec![])]);
        let reqs = ResourceRequirements {
            cpu_millicores: 1000,
            memory_mb: 9000,
            gpus: None,
        };
        assert!(find_suitable_node(&snapshot, &reqs).is_none());
    }
}