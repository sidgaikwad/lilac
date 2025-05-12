use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ServiceError;

use super::user::UserId;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct OrganizationId(Uuid);

impl OrganizationId {
    pub fn new(organization_id: Uuid) -> Self {
        Self(organization_id)
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

impl Default for OrganizationId {
    fn default() -> Self {
        Self::generate()
    }
}

impl Display for OrganizationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for OrganizationId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<OrganizationId> for Uuid {
    fn from(value: OrganizationId) -> Self {
        value.into_inner()
    }
}

impl TryFrom<String> for OrganizationId {
    type Error = ServiceError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let id = Uuid::try_parse(&value)
            .map_err(|_| ServiceError::ParseError("OrganizationId".into()))?;
        Ok(Self(id))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Organization {
    pub organization_id: OrganizationId,
    pub organization_name: String,
}

impl Organization {
    pub fn new(organization_id: OrganizationId, organization_name: String) -> Self {
        Self {
            organization_id,
            organization_name,
        }
    }

    pub fn create(organization_name: String) -> Self {
        Self {
            organization_id: OrganizationId::generate(),
            organization_name,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OrganizationMembership {
    pub organization_id: OrganizationId,
    pub user_id: UserId,
    pub joined_at: DateTime<Utc>,
}
