use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ServiceError;

use super::organization::OrganizationId;

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

impl From<Uuid> for ProjectId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for ProjectId {
    type Error = ServiceError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let id =
            Uuid::try_parse(&value).map_err(|_| ServiceError::ParseError("ProjectId".into()))?;
        Ok(Self(id))
    }
}

#[derive(Clone, Debug, Default, sqlx::FromRow)]
pub struct Project {
    pub project_id: ProjectId,
    pub project_name: String,
    pub organization_id: OrganizationId,
}

impl Project {
    pub fn new(
        project_id: ProjectId,
        project_name: String,
        organization_id: OrganizationId,
    ) -> Self {
        Self {
            project_id,
            project_name,
            organization_id,
        }
    }

    pub fn create(project_name: String, organization_id: OrganizationId) -> Self {
        Self {
            project_id: ProjectId::generate(),
            project_name,
            organization_id,
        }
    }
}
