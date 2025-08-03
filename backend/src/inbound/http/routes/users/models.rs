use chrono::{DateTime, Utc};
use secrecy::SecretString;
use serde::Serialize;

use crate::domain::{
    self,
    user::models::{ApiKey, ApiKeyId, NewApiKey, User, UserId},
};

/// The body of a [User] get response.
#[derive(Debug, Clone, Serialize)]
pub struct GetUserHttpResponse {
    pub user_id: UserId,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl From<User> for GetUserHttpResponse {
    fn from(user: User) -> Self {
        Self {
            user_id: user.id,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateApiKeyResponse {
    pub id: ApiKeyId,
    pub prefix: String,
    #[serde(serialize_with = "domain::serialize_secret_string")]
    pub key: SecretString,
    pub created_at: DateTime<Utc>,
}

impl From<NewApiKey> for CreateApiKeyResponse {
    fn from(new_api_key: NewApiKey) -> Self {
        Self {
            id: new_api_key.id,
            prefix: new_api_key.prefix,
            key: new_api_key.key,
            created_at: new_api_key.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ApiKeyResponse {
    pub id: ApiKeyId,
    pub prefix: String,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl From<ApiKey> for ApiKeyResponse {
    fn from(api_key: ApiKey) -> Self {
        Self {
            id: api_key.id,
            prefix: api_key.prefix,
            created_at: api_key.created_at,
            last_used_at: api_key.last_used_at,
            expires_at: api_key.expires_at,
        }
    }
}
