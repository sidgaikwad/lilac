use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::training_job::models::{TrainingJob, TrainingJobStatus};

#[derive(Debug, Deserialize)]
pub struct CreateTrainingJobRequest {
    pub name: String,
    pub definition: String,
    pub cluster_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct CreateTrainingJobResponse {
    pub id: Uuid,
    pub name: String,
    pub status: TrainingJobStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<TrainingJob> for CreateTrainingJobResponse {
    fn from(training_job: TrainingJob) -> Self {
        Self {
            id: training_job.id,
            name: training_job.name,
            status: training_job.status,
            created_at: training_job.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateTrainingJobStatusRequest {
    pub status: TrainingJobStatus,
}

#[derive(Debug, Deserialize)]
pub struct PostLogsRequest {
    pub logs: String,
}