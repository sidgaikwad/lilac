use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::training_job::models::{TrainingJob, TrainingJobStatus, TrainingJobWithTargets};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ClusterTargetRequest {
    pub cluster_id: Uuid,
    pub priority: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateTrainingJobRequest {
    pub name: String,
    pub definition: String,
    #[serde(default = "default_priority")]
    pub priority: i32,
    pub resource_requirements: serde_json::Value,
    pub targets: Vec<ClusterTargetRequest>,
}

fn default_priority() -> i32 {
    100
}

#[derive(Debug, Serialize)]
pub struct CreateTrainingJobResponse {
    #[serde(flatten)]
    pub job: TrainingJob,
    pub targets: Vec<ClusterTargetRequest>,
}

impl From<TrainingJobWithTargets> for CreateTrainingJobResponse {
    fn from(training_job_with_targets: TrainingJobWithTargets) -> Self {
        let targets = training_job_with_targets
            .targets
            .into_iter()
            .map(|t| ClusterTargetRequest {
                cluster_id: t.cluster_id,
                priority: t.priority,
            })
            .collect();

        Self {
            job: training_job_with_targets.job,
            targets,
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