use serde::{Deserialize, Serialize};

use super::{project::ProjectId, user::UserId};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ProjectMembership {
    pub project_id: ProjectId,
    pub user_id: UserId,
    pub role: Option<String>,
}