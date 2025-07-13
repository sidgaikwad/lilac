use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    model::{
        dataset::{S3Bucket, SnowflakeConnector},
        project::ProjectId,
    },
    ServiceError,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct DatasetId(Uuid);

impl DatasetId {
    pub fn new(dataset_id: Uuid) -> Self {
        Self(dataset_id)
    }

    pub fn generate() -> Self {
        let id = Uuid::new_v4();
        Self(id)
    }

    pub fn inner(&self) -> &Uuid {
        &self.0
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Default for DatasetId {
    fn default() -> Self {
        Self::generate()
    }
}

impl Display for DatasetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for DatasetId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<DatasetId> for Uuid {
    fn from(value: DatasetId) -> Self {
        value.into_inner()
    }
}

impl TryFrom<String> for DatasetId {
    type Error = ServiceError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let id = Uuid::try_parse(&value).map_err(|_| {
            ServiceError::InternalError(format!("failed to parse dataset ID: {value}"))
        })?;
        Ok(Self(id))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(tag = "source_type")]
pub enum DatasetSource {
    #[default]
    Unknown,
    S3(S3Bucket),
    Snowflake(SnowflakeConnector),
}

impl DatasetSource {
    pub fn get_type(&self) -> String {
        match self {
            DatasetSource::Unknown => "Unknown".to_string(),
            DatasetSource::S3(_) => "S3".to_string(),
            DatasetSource::Snowflake(_) => "Snowflake".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dataset {
    pub dataset_id: DatasetId,
    pub dataset_name: String,
    pub description: Option<String>,
    pub project_id: ProjectId,
    pub dataset_source: DatasetSource,
}

impl Dataset {
    pub fn new(
        dataset_id: DatasetId,
        dataset_name: String,
        description: Option<String>,
        project_id: ProjectId,
        dataset_source: DatasetSource,
    ) -> Self {
        Self {
            dataset_id,
            dataset_name,
            description,
            project_id,
            dataset_source,
        }
    }

    pub fn create(
        dataset_name: String,
        description: Option<String>,
        project_id: ProjectId,
        dataset_source: DatasetSource,
    ) -> Self {
        Self {
            dataset_id: DatasetId::generate(),
            dataset_name,
            description,
            project_id,
            dataset_source,
        }
    }
}
