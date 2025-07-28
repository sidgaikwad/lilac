#[cfg(test)]
use mockall::automock;
use async_trait::async_trait;
use super::models::{GetTrainingJobsFilters, TrainingJob, TrainingJobStatus};
use crate::inbound::http::routes::training_jobs::models::CreateTrainingJobRequest;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TrainingJobRepository: Send + Sync {
    async fn create(&self, training_job: &TrainingJob) -> Result<(), anyhow::Error>;
    async fn get_training_jobs(
        &self,
        filters: GetTrainingJobsFilters,
    ) -> Result<Vec<TrainingJob>, anyhow::Error>;
    async fn get_queued_jobs_for_queue(&self, queue_id: Uuid) -> Result<Vec<TrainingJob>, anyhow::Error>;
    async fn update_status(&self, id: Uuid, status: TrainingJobStatus) -> Result<(), anyhow::Error>;
    async fn mark_as_starting(&self, id: Uuid, cluster_id: Uuid) -> Result<(), anyhow::Error>;
    async fn post_logs(&self, id: Uuid, logs: String) -> Result<(), anyhow::Error>;
}

#[async_trait]
pub trait TrainingJobService: Send + Sync {
    async fn create(
        &self,
        request: CreateTrainingJobRequest,
    ) -> Result<TrainingJob, anyhow::Error>;
    async fn get_training_jobs(
        &self,
        filters: GetTrainingJobsFilters,
    ) -> Result<Vec<TrainingJob>, anyhow::Error>;
    async fn update_status(&self, id: Uuid, status: TrainingJobStatus) -> Result<(), anyhow::Error>;
    async fn mark_as_starting(&self, id: Uuid, cluster_id: Uuid) -> Result<(), anyhow::Error>;
    async fn post_logs(&self, id: Uuid, logs: String) -> Result<(), anyhow::Error>;
}