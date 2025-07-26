use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::training_job::models::TrainingJob;

use super::models::ClusterSnapshot;

#[derive(Debug, Default)]
pub struct PlatformCapabilities {
    pub supports_gpus: bool,
}

#[async_trait]
pub trait ComputePlatform: Send + Sync {
    fn platform_type(&self) -> &'static str;
    fn capabilities(&self) -> PlatformCapabilities;
    async fn get_cluster_snapshot(
        &self,
        cluster_id: &Uuid,
    ) -> Result<ClusterSnapshot, anyhow::Error>;
    async fn allocate_job(
        &self,
        job: &TrainingJob,
        cluster_id: &Uuid,
        node_name: &str,
    ) -> Result<(), anyhow::Error>;
    async fn deallocate_job(&self, job_id: &Uuid) -> Result<(), anyhow::Error>;
}