use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ServiceError;

use super::{dataset::DatasetId, pipeline::PipelineId};

#[derive(Clone, Debug, strum::EnumString, strum::Display)]
#[strum(serialize_all = "snake_case")]
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
    pub input_dataset_id: DatasetId,
}

impl Job {
    pub fn new(
        job_id: JobId,
        pipeline_id: PipelineId,
        status: JobStatus,
        input_dataset_id: DatasetId,
    ) -> Self {
        Self {
            job_id,
            pipeline_id,
            status,
            input_dataset_id,
        }
    }

    pub fn create(pipeline_id: PipelineId, input_dataset_id: DatasetId) -> Self {
        Self {
            job_id: JobId::generate(),
            pipeline_id,
            status: JobStatus::Pending,
            input_dataset_id,
        }
    }
}
