use serde::Serialize;

use crate::domain::user::models::{User, UserId};

/// The body of a [User] get response.
#[derive(Debug, Clone, Serialize)]
pub struct GetUserHttpResponse {
    pub user_id: UserId,
    pub name: String,
    pub email: String,
}

impl From<User> for GetUserHttpResponse {
    fn from(user: User) -> Self {
        Self {
            user_id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}
