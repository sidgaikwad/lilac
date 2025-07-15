use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Copy, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(pub Uuid);

impl From<UserId> for Uuid {
    fn from(user_id: UserId) -> Self {
        user_id.0
    }
}

impl From<uuid::Uuid> for UserId {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub password_hash: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    // TODO: A user's name should be generated from the user's email address if not provided.
    pub name: Option<String>,
    #[validate(length(min = 8))]
    pub password: Option<String>,
}