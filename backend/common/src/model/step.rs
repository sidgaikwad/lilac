use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ServiceError;

use super::{pipeline::PipelineId, step_definition::StepDefinitionId};

#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct StepId(Uuid);

impl StepId {
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

impl Default for StepId {
    fn default() -> Self {
        Self::generate()
    }
}

impl From<Uuid> for StepId {
    fn from(value: Uuid) -> Self {
        println!("{value}");
        Self(value)
    }
}

impl From<StepId> for Uuid {
    fn from(value: StepId) -> Self {
        value.into_inner()
    }
}

impl TryFrom<String> for StepId {
    type Error = ServiceError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let id = Uuid::try_parse(&value).map_err(|_| ServiceError::ParseError("StepId".into()))?;
        Ok(Self(id))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    pub step_id: StepId,
    pub step_definition_id: StepDefinitionId,
    pub pipeline_id: PipelineId,
    pub step_parameters: serde_json::Value,
}

impl Step {
    pub fn new(
        step_instance_id: StepId,
        step_definition_id: StepDefinitionId,
        pipeline_id: PipelineId,
        step_parameters: serde_json::Value,
    ) -> Self {
        Self {
            step_id: step_instance_id,
            step_definition_id,
            pipeline_id,
            step_parameters,
        }
    }

    pub fn create(
        step_definition_id: StepDefinitionId,
        pipeline_id: PipelineId,
        step_parameters: serde_json::Value,
    ) -> Self {
        Self {
            step_id: StepId::generate(),
            step_definition_id,
            pipeline_id,
            step_parameters,
        }
    }
}
