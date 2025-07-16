use async_trait::async_trait;
use thiserror::Error;

use super::models::{CreateDatasetRequest, Dataset, DatasetId, DatasetSource};

#[derive(Debug, Error)]
pub enum DatasetRepositoryError {
    #[error("dataset with id {0} not found")]
    NotFound(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[async_trait]
pub trait DatasetRepository: Send + Sync + 'static {
    async fn create_dataset(
        &self,
        req: &CreateDatasetRequest,
    ) -> Result<Dataset, DatasetRepositoryError>;
    async fn get_dataset_by_id(&self, id: &DatasetId) -> Result<Dataset, DatasetRepositoryError>;
    async fn list_datasets(&self) -> Result<Vec<Dataset>, DatasetRepositoryError>;
    async fn delete_dataset(&self, id: &DatasetId) -> Result<(), DatasetRepositoryError>;
}
#[derive(Debug, Error)]
pub enum DataSourceError {
    #[error("invalid connection details: {0}")]
    InvalidConnection(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[async_trait]
pub trait DataSourceTester: Send + Sync {
    async fn test_connection(&self, source: &DatasetSource) -> Result<(), DataSourceError>;
}
