use crate::{domain::serialize_secret_string, identifier};
use chrono::{DateTime, Utc};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

identifier!(DatasetId);

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
