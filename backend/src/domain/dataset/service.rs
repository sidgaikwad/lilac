use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::dataset::{
    models::{CreateDatasetRequest, Dataset, DatasetId, DatasetSource},
    ports::{DataSourceError, DataSourceTester, DatasetRepository, DatasetRepositoryError},
};

#[derive(Debug, thiserror::Error)]
pub enum DatasetServiceError {
    #[error("invalid permissions")]
    InvalidPermissions,
    #[error("dataset with {field} {value} already exists")]
    DatasetExists { field: String, value: String },
    #[error("dataset {0} not found")]
    DatasetNotFound(String),
    #[error("could not connect to dataset: {0}")]
    ConnectionError(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<DatasetRepositoryError> for DatasetServiceError {
    fn from(error: DatasetRepositoryError) -> Self {
        match error {
            DatasetRepositoryError::NotFound(id) => Self::DatasetNotFound(id),
            DatasetRepositoryError::Unknown(error) => Self::Unknown(error),
        }
    }
}

impl From<DataSourceError> for DatasetServiceError {
    fn from(error: DataSourceError) -> Self {
        match error {
            DataSourceError::InvalidConnection(err) => Self::ConnectionError(err),
            DataSourceError::Unknown(error) => Self::Unknown(error),
        }
    }
}

#[async_trait]
pub trait DatasetService: Send + Sync {
    async fn create_dataset(
        &self,
        req: &CreateDatasetRequest,
    ) -> Result<Dataset, DatasetServiceError>;
    async fn get_dataset_by_id(&self, id: &DatasetId) -> Result<Dataset, DatasetServiceError>;
    async fn list_datasets(&self) -> Result<Vec<Dataset>, DatasetServiceError>;
    async fn delete_dataset(&self, id: &DatasetId) -> Result<(), DatasetServiceError>;
    async fn test_data_source_connection(
        &self,
        source: &DatasetSource,
    ) -> Result<(), DatasetServiceError>;
}

#[derive(Clone)]
pub struct DatasetServiceImpl<R: DatasetRepository, D: DataSourceTester> {
    repo: Arc<R>,
    data_source: Arc<D>,
}

impl<R: DatasetRepository, D: DataSourceTester> DatasetServiceImpl<R, D> {
    pub fn new(repo: Arc<R>, data_source: Arc<D>) -> Self {
        Self { repo, data_source }
    }
}

#[async_trait]
impl<R: DatasetRepository, D: DataSourceTester> DatasetService for DatasetServiceImpl<R, D> {
    async fn create_dataset(
        &self,
        req: &CreateDatasetRequest,
    ) -> Result<Dataset, DatasetServiceError> {
        self.data_source.test_connection(&req.source).await?;
        self.repo.create_dataset(req).await.map_err(|e| e.into())
    }

    async fn get_dataset_by_id(&self, id: &DatasetId) -> Result<Dataset, DatasetServiceError> {
        self.repo.get_dataset_by_id(id).await.map_err(|e| e.into())
    }

    async fn list_datasets(&self) -> Result<Vec<Dataset>, DatasetServiceError> {
        self.repo.list_datasets().await.map_err(|e| e.into())
    }

    async fn delete_dataset(&self, id: &DatasetId) -> Result<(), DatasetServiceError> {
        self.repo.delete_dataset(id).await.map_err(|e| e.into())
    }

    async fn test_data_source_connection(
        &self,
        source: &DatasetSource,
    ) -> Result<(), DatasetServiceError> {
        Ok(self.data_source.test_connection(source).await?)
    }
}
