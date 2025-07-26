use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    cluster::{models::ClusterId, ports::ClusterRepository},
    scheduler::{
        models::ClusterSnapshot,
        ports::{ComputePlatform, PlatformCapabilities},
    },
    training_job::models::TrainingJob,
};

use super::factory::KubeClientFactory;

pub struct KubernetesPlugin {
    kube_client_factory: Arc<dyn KubeClientFactory>,
    cluster_repo: Arc<dyn ClusterRepository>,
}

impl KubernetesPlugin {
    pub fn new(
        kube_client_factory: Arc<dyn KubeClientFactory>,
        cluster_repo: Arc<dyn ClusterRepository>,
    ) -> Self {
        Self {
            kube_client_factory,
            cluster_repo,
        }
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

    async fn get_cluster_snapshot(
        &self,
        cluster_id: &Uuid,
    ) -> Result<ClusterSnapshot, anyhow::Error> {
        let cluster = self
            .cluster_repo
            .get_cluster_by_id(&(*cluster_id).into())
            .await?;
        let _kube_client = self.kube_client_factory.create_client(&cluster).await?;

        // TODO: Implement communication with Kubernetes API server to get node resources
        println!("Getting snapshot for cluster {}", cluster_id);
        Ok(ClusterSnapshot {
            cluster_id: *cluster_id,
            nodes: vec![],
        })
    }

    async fn allocate_job(
        &self,
        job: &TrainingJob,
        cluster_id: &Uuid,
        node_name: &str,
    ) -> Result<(), anyhow::Error> {
        let cluster = self
            .cluster_repo
            .get_cluster_by_id(&(*cluster_id).into())
            .await?;
        let _kube_client = self.kube_client_factory.create_client(&cluster).await?;

        // TODO: Implement job allocation (e.g., creating a Pod)
        println!(
            "Allocating job {} to node {} on cluster {}",
            job.id, node_name, cluster_id
        );
        Ok(())
    }

    async fn deallocate_job(&self, job_id: &Uuid) -> Result<(), anyhow::Error> {
        // TODO: Implement job deallocation (e.g., deleting a Pod)
        println!("Deallocating job {}", job_id);
        Ok(())
    }
}