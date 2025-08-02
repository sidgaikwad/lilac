use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{domain::user::models::UserId, identifier};

identifier!(ProjectId);

#[derive(Clone, Debug)]
pub struct Project {
    pub id: ProjectId,
    pub owner_id: UserId,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct CreateProjectRequest {
    pub owner_id: UserId,
    pub name: String,
}
