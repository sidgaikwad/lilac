use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{
    queue::models::QueueId,
    training_job::models::{TrainingJob, TrainingJobStatus},
};

#[derive(Debug, Deserialize)]
pub struct CreateTrainingJobRequest {
    pub name: String,
    pub definition: String,
    pub queue_id: QueueId,
    pub resource_requirements: serde_json::Value,
}

pub type CreateTrainingJobResponse = TrainingJob;

#[derive(Debug, Deserialize)]
pub struct UpdateTrainingJobStatusRequest {
    pub status: TrainingJobStatus,
}

#[derive(Debug, Deserialize)]
pub struct PostLogsRequest {
    pub logs: String,
}
