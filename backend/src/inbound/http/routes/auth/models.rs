use secrecy::SecretString;
use serde::{Deserialize, Serialize};

use crate::domain::user::models::{CreateUserRequest, UserId};

#[derive(Clone, Debug, Deserialize)]
pub struct LoginHttpRequest {
    pub email: String,
    pub password: SecretString,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SignUpHttpRequest {
    pub email: String,
    pub name: String,
    pub password: SecretString,
}

impl From<SignUpHttpRequest> for CreateUserRequest {
    fn from(value: SignUpHttpRequest) -> Self {
        Self {
            email: value.email,
            name: Some(value.name),
            password: value.password,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct SignUpHttpResponse {
    pub user_id: UserId,
}
