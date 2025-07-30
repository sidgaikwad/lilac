use super::models::{GetTrainingJobsFilters, TrainingJob, TrainingJobStatus};
use crate::{
    domain::{cluster::models::ClusterId, queue::models::QueueId, training_job::models::JobId},
    inbound::http::routes::training_jobs::models::CreateTrainingJobRequest,
};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TrainingJobRepository: Send + Sync {
    async fn create(&self, training_job: &TrainingJob) -> Result<(), anyhow::Error>;
    async fn get_training_jobs(
        &self,
        filters: GetTrainingJobsFilters,
    ) -> Result<Vec<TrainingJob>, anyhow::Error>;
    async fn get_queued_jobs_for_queue(
        &self,
        queue_id: &QueueId,
    ) -> Result<Vec<TrainingJob>, anyhow::Error>;
    async fn update_status(
        &self,
        id: &JobId,
        status: TrainingJobStatus,
    ) -> Result<(), anyhow::Error>;
    async fn mark_as_starting(
        &self,
        id: &JobId,
        cluster_id: &ClusterId,
    ) -> Result<(), anyhow::Error>;
    async fn post_logs(&self, id: &JobId, logs: String) -> Result<(), anyhow::Error>;
}

#[async_trait]
pub trait TrainingJobService: Send + Sync {
    async fn create(&self, request: CreateTrainingJobRequest)
        -> Result<TrainingJob, anyhow::Error>;
    async fn get_training_jobs(
        &self,
        filters: GetTrainingJobsFilters,
    ) -> Result<Vec<TrainingJob>, anyhow::Error>;
    async fn update_status(
        &self,
        id: &JobId,
        status: TrainingJobStatus,
    ) -> Result<(), anyhow::Error>;
    async fn mark_as_starting(
        &self,
        id: &JobId,
        cluster_id: &ClusterId,
    ) -> Result<(), anyhow::Error>;
    async fn post_logs(&self, id: &JobId, logs: String) -> Result<(), anyhow::Error>;
}
