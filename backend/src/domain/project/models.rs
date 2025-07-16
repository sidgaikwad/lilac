use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::user::models::UserId;

#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct ProjectId(pub Uuid);
impl From<Uuid> for ProjectId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Project {
    pub id: ProjectId,
    pub owner_id: UserId,
    pub name: String,
    pub created_at:  DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Validate)]
pub struct CreateProjectRequest {
    // TODO: Generate a proper owner ID
    pub owner_id: Option<UserId>,
    #[validate(length(min = 1))]
    pub name: String,
}
