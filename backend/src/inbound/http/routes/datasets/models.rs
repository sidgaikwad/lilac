use secrecy::SecretString;
use serde::{Deserialize, Serialize};

use crate::domain::{
    dataset::models::{
        CreateDatasetRequest, Dataset, DatasetId, DatasetSource, S3Bucket, Snowflake,
    },
    serialize_secret_string,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpDatasetSource {
    S3 {
        bucket_name: String,
        access_key: String,
        #[serde(serialize_with = "serialize_secret_string")]
        secret_key: SecretString,
    },
    Snowflake {
        username: String,
        #[serde(serialize_with = "serialize_secret_string")]
        password: SecretString,
        account: String,
        warehouse: Option<String>,
        database: Option<String>,
        schema: Option<String>,
        role: Option<String>,
    },
}

impl From<HttpDatasetSource> for DatasetSource {
    fn from(value: HttpDatasetSource) -> Self {
        match value {
            HttpDatasetSource::S3 {
                bucket_name,
                access_key,
                secret_key,
            } => Self::S3(S3Bucket {
                bucket: bucket_name,
                access_key,
                secret_key,
            }),
            HttpDatasetSource::Snowflake {
                username,
                password,
                account,
                warehouse,
                database,
                schema,
                role,
            } => Self::Snowflake(Snowflake {
                username,
                password,
                account,
                warehouse,
                database,
                schema,
                role,
            }),
        }
    }
}

impl From<DatasetSource> for HttpDatasetSource {
    fn from(value: DatasetSource) -> Self {
        match value {
            DatasetSource::S3(S3Bucket {
                bucket,
                access_key,
                secret_key,
            }) => Self::S3 {
                bucket_name: bucket,
                access_key,
                secret_key,
            },
            DatasetSource::Snowflake(Snowflake {
                username,
                password,
                account,
                warehouse,
                database,
                schema,
                role,
            }) => Self::Snowflake {
                username,
                password,
                account,
                warehouse,
                database,
                schema,
                role,
            },
        }
    }
}

/// The body of a [Dataset] creation request.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateDatasetHttpRequest {
    dataset_name: String,
    dataset_description: Option<String>,
    data_source: HttpDatasetSource,
}

impl From<CreateDatasetHttpRequest> for CreateDatasetRequest {
    fn from(value: CreateDatasetHttpRequest) -> Self {
        CreateDatasetRequest {
            name: value.dataset_name,
            description: value.dataset_description,
            source: value.data_source.into(),
        }
    }
}

/// The body of a [Dataset] creation response.
#[derive(Debug, Clone, Serialize)]
pub struct CreateDatasetHttpResponse {
    pub dataset_id: DatasetId,
}

/// The body of a [Dataset] get response.
#[derive(Debug, Clone, Serialize)]
pub struct GetDatasetHttpResponse {
    pub dataset_id: DatasetId,
    pub dataset_name: String,
    pub dataset_description: Option<String>,
    pub dataset_source: HttpDatasetSource,
}

impl From<Dataset> for GetDatasetHttpResponse {
    fn from(value: Dataset) -> Self {
        Self {
            dataset_id: value.id,
            dataset_name: value.name,
            dataset_description: value.description,
            dataset_source: value.source.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct HttpDatasetSummary {
    pub dataset_id: DatasetId,
    pub dataset_name: String,
    pub dataset_description: Option<String>,
    pub source_type: String,
}

impl From<Dataset> for HttpDatasetSummary {
    fn from(dataset: Dataset) -> Self {
        let source_type = match dataset.source {
            DatasetSource::S3(_) => "S3".to_string(),
            DatasetSource::Snowflake(_) => "Snowflake".to_string(),
        };

        Self {
            dataset_id: dataset.id,
            dataset_name: dataset.name,
            dataset_description: dataset.description,
            source_type,
        }
    }
}

/// The body of a [Dataset] list response.
#[derive(Clone, Debug, Serialize)]
pub struct ListDatasetsHttpResponse {
    pub datasets: Vec<HttpDatasetSummary>,
}

impl From<Vec<Dataset>> for ListDatasetsHttpResponse {
    fn from(value: Vec<Dataset>) -> Self {
        Self {
            datasets: value.into_iter().map(HttpDatasetSummary::from).collect(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct TestConnectionHttpResponse {
    pub success: bool,
}
