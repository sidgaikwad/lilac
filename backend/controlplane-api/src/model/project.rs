use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ServiceError;

use super::integration::AWSIntegration;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ProjectId(Uuid);

impl ProjectId {
    pub fn new(project_id: Uuid) -> Self {
        Self(project_id)
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

impl Default for ProjectId {
    fn default() -> Self {
        Self::generate()
    }
}

impl Display for ProjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for ProjectId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for ProjectId {
    type Error = ServiceError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let id = Uuid::try_parse(&value).map_err(|_| {
            ServiceError::InternalError(format!("failed to parse project ID: {value}"))
        })?;
        Ok(Self(id))
    }
}

#[derive(Clone, Debug, Default, sqlx::FromRow)]
pub struct Project {
    pub project_id: ProjectId,
    pub project_name: String,
    #[sqlx(json(nullable))]
    pub aws_integration: Option<AWSIntegration>,
}

impl Project {
    pub fn new(
        project_id: ProjectId,
        project_name: String,
        aws_integration: Option<AWSIntegration>,
    ) -> Self {
        Self {
            project_id,
            project_name,
            aws_integration,
        }
    }

    pub fn create(project_name: String) -> Self {
        Self {
            project_id: ProjectId::generate(),
            project_name,
            aws_integration: None,
        }
    }
}
