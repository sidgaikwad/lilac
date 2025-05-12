use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ServiceError;

use super::project::ProjectId;

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
        let id =
            Uuid::try_parse(&value).map_err(|_| ServiceError::ParseError("DatasetId".into()))?;
        Ok(Self(id))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dataset {
    pub dataset_id: DatasetId,
    pub dataset_name: String,
    pub description: Option<String>,
    pub project_id: ProjectId,
    pub dataset_path: String,
}

impl Dataset {
    pub fn new(
        dataset_id: DatasetId,
        dataset_name: String,
        description: Option<String>,
        project_id: ProjectId,
        dataset_path: String,
    ) -> Self {
        Self {
            dataset_id,
            dataset_name,
            description,
            project_id,
            dataset_path,
        }
    }

    pub fn create(
        dataset_name: String,
        description: Option<String>,
        project_id: ProjectId,
        dataset_path: String,
    ) -> Self {
        Self {
            dataset_id: DatasetId::generate(),
            dataset_name,
            description,
            project_id,
            dataset_path,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct DatasetFileMetadata {
    pub file_name: String,
    pub file_type: String,
    pub size: i64,
    pub created_at: DateTime<Utc>,
    pub url: String,
}

impl DatasetFileMetadata {
    pub fn new(file_name: String, file_type: String, size: i64, created_at: DateTime<Utc>, url: String) -> Self {
        Self {
            file_name,
            file_type,
            size,
            created_at,
            url,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct DatasetFile {
    pub metadata: DatasetFileMetadata,
    pub contents: Vec<u8>,
}

impl DatasetFile {
    pub fn new(file_name: String, file_type: String, size: i64, created_at: DateTime<Utc>, url: String, contents: Vec<u8>) -> Self {
        Self {
            metadata: DatasetFileMetadata {
                file_name,
                file_type,
                size,
                created_at,
                url,
            },
            contents,
        }
    }
}
