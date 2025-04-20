use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ServiceError;

use super::pipeline::PipelineId;

#[derive(Clone, Debug, strum::EnumString, strum::Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum JobStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct JobId(Uuid);

impl JobId {
    pub fn new(job_id: Uuid) -> Self {
        Self(job_id)
    }

    pub fn generate() -> Self {
        let id = Uuid::new_v4();
        Self(id)
    }

    pub fn inner(&self) -> &Uuid {
        &self.0
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Default for JobId {
    fn default() -> Self {
        Self::generate()
    }
}

impl From<Uuid> for JobId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<JobId> for Uuid {
    fn from(value: JobId) -> Self {
        value.into_inner()
    }
}

impl TryFrom<String> for JobId {
    type Error = ServiceError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let id = Uuid::try_parse(&value).map_err(|_| ServiceError::ParseError("JobId".into()))?;
        Ok(Self(id))
    }
}

pub struct Job {
    pub job_id: JobId,
    pub pipeline_id: PipelineId,
    pub status: JobStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
}

impl Job {
    pub fn new(
        job_id: JobId,
        pipeline_id: PipelineId,
        status: JobStatus,
        created_at: DateTime<Utc>,
        started_at: Option<DateTime<Utc>>,
        ended_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            job_id,
            pipeline_id,
            status,
            created_at,
            started_at,
            ended_at,
        }
    }

    pub fn create(pipeline_id: PipelineId) -> Self {
        Self {
            job_id: JobId::generate(),
            pipeline_id,
            status: JobStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            ended_at: None,
        }
    }
}
