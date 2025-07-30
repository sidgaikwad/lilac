use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::training_job::models::{ResourceRequirements, TrainingJob};

#[derive(Debug, Clone)]
pub struct SchedulingDecision {
    pub cluster_id: Uuid,
    pub node_name: String,
}

#[derive(Debug, Default)]
pub struct PlatformCapabilities {
    pub supports_gpus: bool,
}

#[async_trait]
pub trait ComputePlatform: Send + Sync {
    fn platform_type(&self) -> &'static str;
    fn capabilities(&self) -> PlatformCapabilities;
    async fn find_suitable_node(
        &self,
        cluster_id: &Uuid,
        requirements: &ResourceRequirements,
    ) -> Result<Option<SchedulingDecision>, anyhow::Error>;
    async fn allocate_job(
        &self,
        job: &TrainingJob,
        decision: &SchedulingDecision,
    ) -> Result<(), anyhow::Error>;
    async fn deallocate_job(&self, job_id: &Uuid) -> Result<(), anyhow::Error>;
}
