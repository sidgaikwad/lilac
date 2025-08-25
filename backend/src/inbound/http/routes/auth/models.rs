use secrecy::SecretString;
use serde::{Deserialize, Serialize};

use crate::domain::user::models::{CreateUserRequest, UserId};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginHttpRequest {
    pub username: String,
    #[serde(serialize_with = "crate::domain::serialize_secret_string")]
    pub password: SecretString,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignUpHttpRequest {
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[serde(serialize_with = "crate::domain::serialize_secret_string")]
    pub password: SecretString,
}

impl From<SignUpHttpRequest> for CreateUserRequest {
    fn from(value: SignUpHttpRequest) -> Self {
        Self {
            username: value.username,
            first_name: value.first_name,
            last_name: value.last_name,
            password: value.password,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignUpHttpResponse {
    pub user_id: UserId,
}
