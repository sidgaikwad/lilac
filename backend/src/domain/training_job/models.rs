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

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TrainingJob {
    pub id: Uuid,
    pub name: String,
    pub definition: String,
    pub status: TrainingJobStatus,
    pub cluster_id: Uuid,
    pub instance_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
pub struct GetTrainingJobsFilters {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub status: Option<TrainingJobStatus>,
    pub cluster_id: Option<Uuid>,
}