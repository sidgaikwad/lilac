use std::sync::Arc;

use async_trait::async_trait;

use super::{
    models::{Cluster, ClusterId, CreateClusterRequest},
    ports::{ClusterRepository, ClusterRepositoryError},
};

#[derive(Debug, thiserror::Error)]
pub enum ClusterServiceError {
    #[error("invalid permissions")]
    InvalidPermissions,
    #[error("cluster with {field} {value} already exists")]
    ClusterExists { field: String, value: String },
    #[error("cluster {0} not found")]
    ClusterNotFound(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<ClusterRepositoryError> for ClusterServiceError {
    fn from(error: ClusterRepositoryError) -> Self {
        match error {
            ClusterRepositoryError::Duplicate { field, value } => {
                Self::ClusterExists { field, value }
            }
            ClusterRepositoryError::NotFound(id) => Self::ClusterNotFound(id),
            ClusterRepositoryError::Unknown(error) => Self::Unknown(error),
        }
    }
}

#[async_trait]
pub trait ClusterService: Send + Sync {
    async fn create_cluster(
        &self,
        req: &CreateClusterRequest,
    ) -> Result<Cluster, ClusterServiceError>;
    async fn get_cluster_by_id(&self, id: &ClusterId) -> Result<Cluster, ClusterServiceError>;
    async fn list_clusters(&self) -> Result<Vec<Cluster>, ClusterServiceError>;
    async fn delete_cluster(&self, id: &ClusterId) -> Result<(), ClusterServiceError>;
}

#[derive(Clone)]
pub struct ClusterServiceImpl<R: ClusterRepository> {
    repo: Arc<R>,
}

impl<R: ClusterRepository> ClusterServiceImpl<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: ClusterRepository> ClusterService for ClusterServiceImpl<R> {
    async fn create_cluster(
        &self,
        req: &CreateClusterRequest,
    ) -> Result<Cluster, ClusterServiceError> {
        Ok(self.repo.create_cluster(req).await?)
    }

    async fn get_cluster_by_id(&self, id: &ClusterId) -> Result<Cluster, ClusterServiceError> {
        Ok(self.repo.get_cluster_by_id(id).await?)
    }

    async fn list_clusters(&self) -> Result<Vec<Cluster>, ClusterServiceError> {
        Ok(self.repo.list_clusters().await?)
    }

    async fn delete_cluster(&self, id: &ClusterId) -> Result<(), ClusterServiceError> {
        Ok(self.repo.delete_cluster(id).await?)
    }
}
