use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "training_job_status", rename_all = "lowercase")]
pub enum TrainingJobStatus {
    Queued,
    Starting,
    Running,
    Succeeded,
    Failed,
}

/// Describes a specific requirement for a GPU.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuRequirement {
    /// The number of GPUs required.
    pub count: i32,
    /// The specific model of the GPU (e.g., "A100", "V100", "RTX4090").
    /// If None, any GPU model is acceptable.
    pub model: Option<String>,
    /// The minimum required memory for each GPU in gigabytes (e.g., 40, 80).
    /// If None, any GPU memory size is acceptable.
    pub memory_gb: Option<i32>,
}

/// Represents the computational resources required for a training job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub cpu_millicores: i32,
    pub memory_mb: i32,
    /// GPU requirements for the job. If None, the job does not require GPUs.
    pub gpus: Option<GpuRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TrainingJob {
    pub id: Uuid,
    pub name: String,
    pub definition: String,
    pub status: TrainingJobStatus,
    pub instance_id: Option<Uuid>,
    pub priority: i32,
    pub resource_requirements: ResourceRequirements,
    pub scheduled_cluster_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TrainingJobClusterTarget {
    pub job_id: Uuid,
    pub cluster_id: Uuid,
    pub priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingJobWithTargets {
    #[serde(flatten)]
    pub job: TrainingJob,
    pub targets: Vec<TrainingJobClusterTarget>,
}

#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
pub struct GetTrainingJobsFilters {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub status: Option<TrainingJobStatus>,
}