use std::sync::Arc;

use uuid::Uuid;

use super::{
    models::{GetTrainingJobsFilters, TrainingJob, TrainingJobStatus},
    ports::{TrainingJobRepository, TrainingJobService},
};
use crate::{
    domain::{cluster::models::ClusterId, training_job::models::JobId},
    inbound::http::routes::training_jobs::models::CreateTrainingJobRequest,
};
use async_trait::async_trait;

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
    ) -> Result<TrainingJob, anyhow::Error> {
        let job_id = JobId::generate();
        let now = chrono::Utc::now();

        let training_job = TrainingJob {
            id: job_id,
            name: request.name,
            definition: request.definition,
            status: TrainingJobStatus::Queued,
            queue_id: request.queue_id,
            resource_requirements: serde_json::from_value(request.resource_requirements)?,
            scheduled_cluster_id: None,
            created_at: now,
            updated_at: now,
        };

        self.repository.create(&training_job).await?;

        Ok(training_job)
    }

    async fn get_training_jobs(
        &self,
        filters: GetTrainingJobsFilters,
    ) -> Result<Vec<TrainingJob>, anyhow::Error> {
        self.repository.get_training_jobs(filters).await
    }

    async fn update_status(
        &self,
        id: &JobId,
        status: TrainingJobStatus,
    ) -> Result<(), anyhow::Error> {
        self.repository.update_status(id, status).await
    }

    async fn mark_as_starting(
        &self,
        id: &JobId,
        cluster_id: &ClusterId,
    ) -> Result<(), anyhow::Error> {
        self.repository.mark_as_starting(id, cluster_id).await
    }

    async fn post_logs(&self, id: &JobId, logs: String) -> Result<(), anyhow::Error> {
        self.repository.post_logs(id, logs).await
    }
}
