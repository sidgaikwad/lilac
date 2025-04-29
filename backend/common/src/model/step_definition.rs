use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ServiceError;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct StepDefinitionId(Uuid);

impl StepDefinitionId {
    pub fn new(step_definition_id: Uuid) -> Self {
        Self(step_definition_id)
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

impl Default for StepDefinitionId {
    fn default() -> Self {
        Self::generate()
    }
}

impl From<Uuid> for StepDefinitionId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<StepDefinitionId> for Uuid {
    fn from(value: StepDefinitionId) -> Self {
        value.into_inner()
    }
}

impl TryFrom<String> for StepDefinitionId {
    type Error = ServiceError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let id = Uuid::try_parse(&value).map_err(|_| ServiceError::ParseError("StepId".into()))?;
        Ok(Self(id))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StepDefinition {
    pub step_definition_id: StepDefinitionId,
    pub step_type: StepType,
    pub schema: serde_json::Value,
}

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString,
)]
pub enum StepType {
    NoOp,
    BlurDetector,
    ResolutionStandardizer,
    Rotate,
    Flip,
    Grayscale,
    Brightness,
    Contrast,
    AddNoise,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterType {
    String,
    Number,
}
