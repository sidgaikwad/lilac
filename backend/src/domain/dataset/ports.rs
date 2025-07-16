use async_trait::async_trait;
use thiserror::Error;

use super::models::{CreateDatasetRequest, Dataset, DatasetId, DatasetSource};
use crate::domain::project::models::ProjectId;

#[derive(Debug, Error)]
pub enum DatasetRepositoryError {
    #[error("dataset with id {0} not found")]
    NotFound(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum DatasetServiceError {
    #[error(transparent)]
    Repository(#[from] DatasetRepositoryError),
    #[error(transparent)]
    DataSource(#[from] DataSourceError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

use crate::domain::user::models::UserId;

#[async_trait]
pub trait DatasetRepository: Send + Sync + 'static {
    async fn create_dataset(
        &self,
        user_id: &UserId,
        req: &CreateDatasetRequest,
    ) -> Result<Dataset, DatasetRepositoryError>;
    async fn get_dataset_by_id(
        &self,
        user_id: &UserId,
        id: &DatasetId,
    ) -> Result<Dataset, DatasetRepositoryError>;
    async fn list_datasets_by_project_id(
        &self,
        user_id: &UserId,
        project_id: &ProjectId,
    ) -> Result<Vec<Dataset>, DatasetRepositoryError>;
    async fn delete_dataset(
        &self,
        user_id: &UserId,
        id: &DatasetId,
    ) -> Result<(), DatasetRepositoryError>;
}

#[async_trait]
pub trait DatasetService: Send + Sync {
    async fn create_dataset(
        &self,
        user_id: &UserId,
        req: &CreateDatasetRequest,
    ) -> Result<Dataset, DatasetServiceError>;
    async fn get_dataset_by_id(
        &self,
        user_id: &UserId,
        id: &DatasetId,
    ) -> Result<Dataset, DatasetServiceError>;
    async fn list_datasets_by_project_id(
        &self,
        user_id: &UserId,
        project_id: &ProjectId,
    ) -> Result<Vec<Dataset>, DatasetServiceError>;
    async fn delete_dataset(
        &self,
        user_id: &UserId,
        id: &DatasetId,
    ) -> Result<(), DatasetServiceError>;
    async fn test_data_source_connection(
        &self,
        source: &DatasetSource,
    ) -> Result<(), DataSourceError>;
}

#[derive(Debug, Error)]
pub enum DataSourceError {
    #[error("invalid connection details: {0}")]
    InvalidConnection(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[async_trait]
pub trait DataSource: Send + Sync {
    async fn test_connection(&self, source: &DatasetSource) -> Result<(), DataSourceError>;
}
