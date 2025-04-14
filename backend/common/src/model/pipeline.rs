use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ServiceError;

use super::{organization::OrganizationId, step::Step};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct PipelineId(Uuid);

impl PipelineId {
    pub fn new(pipeline_id: Uuid) -> Self {
        Self(pipeline_id)
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

impl Default for PipelineId {
    fn default() -> Self {
        Self::generate()
    }
}

impl From<Uuid> for PipelineId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<PipelineId> for Uuid {
    fn from(value: PipelineId) -> Self {
        value.into_inner()
    }
}

impl TryFrom<String> for PipelineId {
    type Error = ServiceError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let id =
            Uuid::try_parse(&value).map_err(|_| ServiceError::ParseError("PipelineId".into()))?;
        Ok(Self(id))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pipeline {
    pub pipeline_id: PipelineId,
    pub pipeline_name: String,
    pub description: Option<String>,
    pub organization_id: OrganizationId,
    pub steps: Vec<Step>,
    pub created_at: DateTime<Utc>,
}

impl Pipeline {
    pub fn new(
        pipeline_id: PipelineId,
        pipeline_name: String,
        description: Option<String>,
        organization_id: OrganizationId,
        steps: Vec<Step>,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            pipeline_id,
            pipeline_name,
            description,
            organization_id,
            steps,
            created_at,
        }
    }

    pub fn create(
        pipeline_name: String,
        description: Option<String>,
        organization_id: OrganizationId,
    ) -> Self {
        Self {
            pipeline_id: PipelineId::generate(),
            pipeline_name,
            description,
            organization_id,
            steps: Vec::new(),
            created_at: Utc::now(),
        }
    }
}
