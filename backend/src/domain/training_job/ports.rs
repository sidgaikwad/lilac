use super::models::{GetTrainingJobsFilters, TrainingJob, TrainingJobStatus};
use crate::domain::{cluster::models::NodeId, queue::models::QueueId, training_job::models::JobId};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

#[derive(Debug, thiserror::Error)]
pub enum TrainingJobRepositoryError {
    #[error("training job with {field} {value} already exists")]
    Duplicate { field: String, value: String },
    #[error("training job with id {0} not found")]
    NotFound(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TrainingJobRepository: Send + Sync {
    async fn create(&self, training_job: &TrainingJob) -> Result<(), TrainingJobRepositoryError>;
    async fn get_training_jobs(
        &self,
        filters: GetTrainingJobsFilters,
    ) -> Result<Vec<TrainingJob>, TrainingJobRepositoryError>;
    async fn get_queued_jobs_for_queue(
        &self,
        queue_id: &QueueId,
    ) -> Result<Vec<TrainingJob>, TrainingJobRepositoryError>;
    async fn update_status(
        &self,
        id: &JobId,
        status: TrainingJobStatus,
    ) -> Result<(), TrainingJobRepositoryError>;
    async fn mark_as_starting(
        &self,
        id: &JobId,
        node_id: &NodeId,
    ) -> Result<(), TrainingJobRepositoryError>;
    async fn post_logs(&self, id: &JobId, logs: String) -> Result<(), TrainingJobRepositoryError>;
}
