use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::{
    cluster::models::NodeId,
    queue::models::QueueId,
    training_job::models::{JobId, ResourceRequirements, TrainingJob, TrainingJobStatus},
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

/// An HTTP representation of a [TrainingJob].
#[derive(Debug, Clone, Serialize)]
pub struct HttpTrainingJob {
    pub job_id: JobId,
    pub job_name: String,
    pub job_status: TrainingJobStatus,
    pub node_id: Option<NodeId>,
    pub queue_id: QueueId,
    pub resource_requirements: ResourceRequirements,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<TrainingJob> for HttpTrainingJob {
    fn from(job: TrainingJob) -> Self {
        Self {
            job_id: job.id,
            job_name: job.name,
            job_status: job.status,
            node_id: job.node_id,
            queue_id: job.queue_id,
            resource_requirements: job.resource_requirements,
            created_at: job.created_at,
            updated_at: job.updated_at,
        }
    }
}
