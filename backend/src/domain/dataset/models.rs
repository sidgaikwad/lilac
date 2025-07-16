use std::fmt::Display;

use crate::domain::serialize_secret_string;
use chrono::{DateTime, Utc};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DatasetId(Uuid);

impl DatasetId {
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

impl Display for DatasetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for DatasetId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<DatasetId> for Uuid {
    fn from(id: DatasetId) -> Self {
        id.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DatasetSource {
    S3(S3Bucket),
    Snowflake(Snowflake),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct S3Bucket {
    pub bucket: String,
    pub access_key: String,
    #[serde(serialize_with = "serialize_secret_string")]
    pub secret_key: SecretString,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Snowflake {
    pub username: String,
    #[serde(serialize_with = "serialize_secret_string")]
    pub password: SecretString,
    pub account: String,
    pub warehouse: Option<String>,
    pub database: Option<String>,
    pub schema: Option<String>,
    pub role: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Dataset {
    pub id: DatasetId,
    pub name: String,
    pub description: Option<String>,
    pub source: DatasetSource,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct CreateDatasetRequest {
    pub name: String,
    pub description: Option<String>,
    pub source: DatasetSource,
}
