use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ServiceError;

use super::{
    project::ProjectId,
    step::{Step, StepId},
};

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

impl Display for PipelineId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
    pub project_id: ProjectId,
    pub steps: Vec<Step>,
    pub step_connections: Vec<(StepId, StepId)>,
}

impl Pipeline {
    pub fn new(
        pipeline_id: PipelineId,
        pipeline_name: String,
        description: Option<String>,
        project_id: ProjectId,
        steps: Vec<Step>,
        step_connections: Vec<(StepId, StepId)>,
    ) -> Self {
        Self {
            pipeline_id,
            pipeline_name,
            description,
            project_id,
            steps,
            step_connections,
        }
    }

    pub fn create(
        pipeline_name: String,
        description: Option<String>,
        project_id: ProjectId,
    ) -> Self {
        Self {
            pipeline_id: PipelineId::generate(),
            pipeline_name,
            description,
            project_id,
            steps: Vec::new(),
            step_connections: Vec::new(),
        }
    }
}
