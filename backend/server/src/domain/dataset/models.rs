use chrono::{DateTime, Utc};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::project::models::ProjectId;

#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct DatasetId(pub Uuid);

fn serialize_secret_string<S>(secret: &SecretString, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(secret.expose_secret())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "source_type")]
pub enum DatasetSource {
    S3(S3Bucket),
    Snowflake(Snowflake),
}

impl Default for DatasetSource {
    fn default() -> Self {
        DatasetSource::S3(S3Bucket {
            bucket: "".to_string(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct S3Bucket {
    pub bucket: String,
}

use secrecy::ExposeSecret;

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dataset {
    pub id: DatasetId,
    pub project_id: ProjectId,
    pub name: String,
    pub description: Option<String>,
    pub source: DatasetSource,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateDatasetRequest {
    pub project_id: ProjectId,
    pub name: String,
    pub description: Option<String>,
    // TODO: A dataset's source should be determined from the integration type
    pub source: Option<DatasetSource>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DatasetSummary {
    pub id: DatasetId,
    pub project_id: ProjectId,
    pub name: String,
    pub description: Option<String>,
    pub source_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Dataset> for DatasetSummary {
    fn from(dataset: Dataset) -> Self {
        let source_type = match dataset.source {
            DatasetSource::S3(_) => "S3".to_string(),
            DatasetSource::Snowflake(_) => "Snowflake".to_string(),
        };

        Self {
            id: dataset.id,
            project_id: dataset.project_id,
            name: dataset.name,
            description: dataset.description,
            source_type,
            created_at: dataset.created_at,
            updated_at: dataset.updated_at,
        }
    }
}
