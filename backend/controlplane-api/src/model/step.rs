use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ServiceError;

use super::pipeline::PipelineId;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Step {
    pub step_id: StepId,
    pub step_type: StepType,
    pub parameter_definitions: Vec<StepParameter>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepType {
    NoOp,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StepParameter {
    parameter_name: String,
    parameter_type: ParameterType,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterType {
    String,
    Number,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct StepInstanceId(Uuid);

impl StepInstanceId {
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

impl Default for StepInstanceId {
    fn default() -> Self {
        Self::generate()
    }
}

impl From<Uuid> for StepInstanceId {
    fn from(value: Uuid) -> Self {
        println!("{value}");
        Self(value)
    }
}

impl From<StepInstanceId> for Uuid {
    fn from(value: StepInstanceId) -> Self {
        value.into_inner()
    }
}

impl TryFrom<String> for StepInstanceId {
    type Error = ServiceError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let id = Uuid::try_parse(&value).map_err(|_| ServiceError::ParseError("StepId".into()))?;
        Ok(Self(id))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct StepInstance {
    pub step_instance_id: StepInstanceId,
    pub step_id: StepId,
    pub pipeline_id: PipelineId,
    pub previous_step: Option<StepInstanceId>,
    pub next_step: Option<StepInstanceId>,
    pub step_parameters: serde_json::Value,
}

impl StepInstance {
    pub fn new(
        step_instance_id: StepInstanceId,
        step_id: StepId,
        pipeline_id: PipelineId,
        next_step: Option<StepInstanceId>,
        previous_step: Option<StepInstanceId>,
        step_parameters: serde_json::Value,
    ) -> Self {
        Self {
            step_instance_id,
            step_id,
            pipeline_id,
            previous_step,
            next_step,
            step_parameters,
        }
    }

    pub fn create(
        step_id: StepId,
        pipeline_id: PipelineId,
        step_parameters: serde_json::Value,
    ) -> Self {
        Self {
            step_instance_id: StepInstanceId::generate(),
            step_id,
            pipeline_id,
            previous_step: None,
            next_step: None,
            step_parameters,
        }
    }
}