use std::fmt::Display;

use chrono::{DateTime, Utc};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::cluster::models::ClusterId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn inner(&self) -> &Uuid {
        &self.0
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for UserId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<UserId> for Uuid {
    fn from(id: UserId) -> Self {
        id.0
    }
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct CreateUserRequest {
    pub email: String,
    pub name: Option<String>,
    pub password: SecretString,
}

// --- API Key Models ---

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ApiKeyId(Uuid);

impl ApiKeyId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn inner(&self) -> &Uuid {
        &self.0
    }
}

impl Display for ApiKeyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for ApiKeyId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

/// Represents a newly generated API key, including the plaintext secret.
/// This struct is only created once and given to the user. It is not stored.
#[derive(Clone, Debug, Serialize)]
pub struct NewApiKey {
    pub id: ApiKeyId,
    pub prefix: String,
    #[serde(serialize_with = "crate::domain::serialize_secret_string")]
    pub key: SecretString,
    pub created_at: DateTime<Utc>,
}

/// Represents an API key as stored in the database.
/// The actual key is hashed and stored in `key_hash`.
#[derive(Clone, Debug)]
pub struct ApiKey {
    pub id: ApiKeyId,
    pub user_id: Option<UserId>,
    pub cluster_id: Option<ClusterId>,
    pub prefix: String,
    pub key_hash: String,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
}
