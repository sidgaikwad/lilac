use std::sync::Arc;

use uuid::Uuid;

use super::{
    models::{GetTrainingJobsFilters, TrainingJob, TrainingJobStatus},
    ports::{TrainingJobRepository, TrainingJobService},
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
    async fn create(&self, name: String, definition: String, cluster_id: Uuid) -> Result<TrainingJob, anyhow::Error> {
        let training_job = TrainingJob {
            id: Uuid::new_v4(),
            name,
            definition,
            status: TrainingJobStatus::Queued,
            cluster_id,
            instance_id: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
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

    async fn update_status(&self, id: Uuid, status: TrainingJobStatus) -> Result<(), anyhow::Error> {
        self.repository.update_status(id, status).await
    }
    async fn schedule(&self, id: Uuid) -> Result<(), anyhow::Error> {
        // TODO: Implement scheduling logic
        self.repository.schedule(id).await
    }

    async fn post_logs(&self, id: Uuid, logs: String) -> Result<(), anyhow::Error> {
        self.repository.post_logs(id, logs).await
    }
}
