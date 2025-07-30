use std::sync::Arc;

use super::{
    models::{GetTrainingJobsFilters, TrainingJob, TrainingJobStatus},
    ports::TrainingJobRepository,
};
use crate::{
    domain::{
        cluster::models::NodeId,
        training_job::{models::JobId, ports::TrainingJobRepositoryError},
    },
    inbound::http::routes::training_jobs::models::CreateTrainingJobRequest,
};
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum TrainingJobServiceError {
    #[error("training job with {field} {value} already exists")]
    TrainingJobExists { field: String, value: String },
    #[error("training job {0} not found")]
    TrainingJobNotFound(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<TrainingJobRepositoryError> for TrainingJobServiceError {
    fn from(error: TrainingJobRepositoryError) -> Self {
        match error {
            TrainingJobRepositoryError::Duplicate { field, value } => {
                Self::TrainingJobExists { field, value }
            }
            TrainingJobRepositoryError::NotFound(id) => Self::TrainingJobNotFound(id),
            TrainingJobRepositoryError::Unknown(error) => Self::Unknown(error),
        }
    }
}

#[async_trait]
pub trait TrainingJobService: Send + Sync {
    async fn create(
        &self,
        request: CreateTrainingJobRequest,
    ) -> Result<TrainingJob, TrainingJobServiceError>;
    async fn get_training_jobs(
        &self,
        filters: GetTrainingJobsFilters,
    ) -> Result<Vec<TrainingJob>, TrainingJobServiceError>;
    async fn update_status(
        &self,
        id: &JobId,
        status: TrainingJobStatus,
    ) -> Result<(), TrainingJobServiceError>;
    async fn mark_as_starting(
        &self,
        id: &JobId,
        node_id: &NodeId,
    ) -> Result<(), TrainingJobServiceError>;
    async fn post_logs(&self, id: &JobId, logs: String) -> Result<(), TrainingJobServiceError>;
}

pub struct TrainingJobServiceImpl {
    repository: Arc<dyn TrainingJobRepository>,
}

impl TrainingJobServiceImpl {
    pub fn new(repository: Arc<dyn TrainingJobRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl TrainingJobService for TrainingJobServiceImpl {
    async fn create(
        &self,
        request: CreateTrainingJobRequest,
    ) -> Result<TrainingJob, TrainingJobServiceError> {
        let job_id = JobId::generate();
        let now = chrono::Utc::now();

        let training_job = TrainingJob {
            id: job_id,
            name: request.name,
            definition: request.definition,
            status: TrainingJobStatus::Queued,
            node_id: None,
            queue_id: request.queue_id,
            resource_requirements: serde_json::from_value(request.resource_requirements)
                .map_err(|e| anyhow::anyhow!(e))?,
            created_at: now,
            updated_at: now,
        };

        self.repository.create(&training_job).await?;

        Ok(training_job)
    }

    async fn get_training_jobs(
        &self,
        filters: GetTrainingJobsFilters,
    ) -> Result<Vec<TrainingJob>, TrainingJobServiceError> {
        Ok(self.repository.get_training_jobs(filters).await?)
    }

    async fn update_status(
        &self,
        id: &JobId,
        status: TrainingJobStatus,
    ) -> Result<(), TrainingJobServiceError> {
        Ok(self.repository.update_status(id, status).await?)
    }

    async fn mark_as_starting(
        &self,
        id: &JobId,
        node_id: &NodeId,
    ) -> Result<(), TrainingJobServiceError> {
        Ok(self.repository.mark_as_starting(id, node_id).await?)
    }

    async fn post_logs(&self, id: &JobId, logs: String) -> Result<(), TrainingJobServiceError> {
        Ok(self.repository.post_logs(id, logs).await?)
    }
}
