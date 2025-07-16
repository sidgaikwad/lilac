use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::{
    dataset::{
        models::{CreateDatasetRequest, Dataset, DatasetId, DatasetSource},
        ports::{
            DataSource, DataSourceError, DatasetRepository, DatasetService, DatasetServiceError,
        },
    },
    project::models::ProjectId,
};

#[derive(Clone)]
pub struct DatasetServiceImpl<R: DatasetRepository, D: DataSource> {
    repo: Arc<R>,
    data_source: Arc<D>,
}

impl<R: DatasetRepository, D: DataSource> DatasetServiceImpl<R, D> {
    pub fn new(repo: Arc<R>, data_source: Arc<D>) -> Self {
        Self { repo, data_source }
    }
}

use crate::domain::user::models::UserId;

#[async_trait]
impl<R: DatasetRepository, D: DataSource> DatasetService for DatasetServiceImpl<R, D> {
    async fn create_dataset(
        &self,
        user_id: &UserId,
        req: &CreateDatasetRequest,
    ) -> Result<Dataset, DatasetServiceError> {
        if let Some(source) = &req.source {
            self.data_source.test_connection(source).await?;
        }
        self.repo
            .create_dataset(user_id, req)
            .await
            .map_err(|e| e.into())
    }

    async fn get_dataset_by_id(
        &self,
        user_id: &UserId,
        id: &DatasetId,
    ) -> Result<Dataset, DatasetServiceError> {
        self.repo
            .get_dataset_by_id(user_id, id)
            .await
            .map_err(|e| e.into())
    }

    async fn list_datasets_by_project_id(
        &self,
        user_id: &UserId,
        project_id: &ProjectId,
    ) -> Result<Vec<Dataset>, DatasetServiceError> {
        self.repo
            .list_datasets_by_project_id(user_id, project_id)
            .await
            .map_err(|e| e.into())
    }

    async fn delete_dataset(
        &self,
        user_id: &UserId,
        id: &DatasetId,
    ) -> Result<(), DatasetServiceError> {
        self.repo
            .delete_dataset(user_id, id)
            .await
            .map_err(|e| e.into())
    }

    async fn test_data_source_connection(
        &self,
        source: &DatasetSource,
    ) -> Result<(), DataSourceError> {
        self.data_source.test_connection(source).await
    }
}
