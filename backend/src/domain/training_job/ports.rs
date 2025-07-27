#[cfg(test)]
use mockall::automock;
use async_trait::async_trait;
use super::models::{
    GetTrainingJobsFilters, TrainingJob, TrainingJobClusterTarget, TrainingJobStatus,
    TrainingJobWithTargets,
};
use crate::inbound::http::routes::training_jobs::models::CreateTrainingJobRequest;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TrainingJobRepository: Send + Sync {
    async fn create(
        &self,
        training_job: &TrainingJob,
        targets: &[TrainingJobClusterTarget],
    ) -> Result<(), anyhow::Error>;
    async fn get_training_jobs(
        &self,
        filters: GetTrainingJobsFilters,
    ) -> Result<Vec<TrainingJob>, anyhow::Error>;
    async fn get_queued_jobs_with_targets(
        &self,
    ) -> Result<Vec<TrainingJobWithTargets>, anyhow::Error>;
    async fn update_status(&self, id: Uuid, status: TrainingJobStatus) -> Result<(), anyhow::Error>;
    async fn mark_as_starting(&self, id: Uuid, cluster_id: Uuid) -> Result<(), anyhow::Error>;
    async fn schedule(&self, id: Uuid) -> Result<(), anyhow::Error>;
    async fn post_logs(&self, id: Uuid, logs: String) -> Result<(), anyhow::Error>;
}

#[async_trait]
pub trait TrainingJobService: Send + Sync {
    async fn create(
        &self,
        request: CreateTrainingJobRequest,
    ) -> Result<TrainingJobWithTargets, anyhow::Error>;
    async fn get_training_jobs(
        &self,
        filters: GetTrainingJobsFilters,
    ) -> Result<Vec<TrainingJob>, anyhow::Error>;
    async fn update_status(&self, id: Uuid, status: TrainingJobStatus) -> Result<(), anyhow::Error>;
    async fn mark_as_starting(&self, id: Uuid, cluster_id: Uuid) -> Result<(), anyhow::Error>;
    async fn schedule(&self, id: Uuid) -> Result<(), anyhow::Error>;
    async fn post_logs(&self, id: Uuid, logs: String) -> Result<(), anyhow::Error>;
}