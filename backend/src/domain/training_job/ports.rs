#[cfg(test)]
use mockall::automock;
use async_trait::async_trait;
use super::models::GetTrainingJobsFilters;
use uuid::Uuid;

use super::models::{TrainingJob, TrainingJobStatus};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TrainingJobRepository: Send + Sync {
    async fn create(&self, training_job: &TrainingJob) -> Result<(), anyhow::Error>;
    async fn get_training_jobs(&self, filters: GetTrainingJobsFilters) -> Result<Vec<TrainingJob>, anyhow::Error>;
    async fn update_status(&self, id: Uuid, status: TrainingJobStatus) -> Result<(), anyhow::Error>;
    async fn schedule(&self, id: Uuid) -> Result<(), anyhow::Error>;
    async fn post_logs(&self, id: Uuid, logs: String) -> Result<(), anyhow::Error>;
}

#[async_trait]
pub trait TrainingJobService: Send + Sync {
    async fn create(&self, name: String, definition: String, cluster_id: Uuid) -> Result<TrainingJob, anyhow::Error>;
    async fn get_training_jobs(&self, filters: GetTrainingJobsFilters) -> Result<Vec<TrainingJob>, anyhow::Error>;
    async fn update_status(&self, id: Uuid, status: TrainingJobStatus) -> Result<(), anyhow::Error>;
    async fn schedule(&self, id: Uuid) -> Result<(), anyhow::Error>;
    async fn post_logs(&self, id: Uuid, logs: String) -> Result<(), anyhow::Error>;
}