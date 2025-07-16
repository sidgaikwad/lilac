use async_trait::async_trait;
use std::sync::Arc;
use validator::Validate;

use super::{
    models::{CreateClusterRequest, Cluster, ClusterId},
    ports::{ClusterRepository, ClusterRepositoryError},
};

#[async_trait]
pub trait ClusterService: Send + Sync {
    async fn create_cluster(
        &self,
        req: &CreateClusterRequest,
    ) -> Result<Cluster, ClusterRepositoryError>;
    async fn get_cluster_by_id(
        &self,
        id: &ClusterId,
    ) -> Result<Cluster, ClusterRepositoryError>;
    async fn list_clusters(
        &self,
    ) -> Result<Vec<Cluster>, ClusterRepositoryError>;
    async fn delete_cluster(
        &self,
        id: &ClusterId,
    ) -> Result<(), ClusterRepositoryError>;
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
    ) -> Result<Cluster, ClusterRepositoryError> {
        req.validate()
            .map_err(|e| ClusterRepositoryError::InvalidInput(e.to_string()))?;

        self.repo.create_cluster(req).await
    }

    async fn get_cluster_by_id(
        &self,
        id: &ClusterId,
    ) -> Result<Cluster, ClusterRepositoryError> {
        self.repo.get_cluster_by_id(id).await
    }

    async fn list_clusters(
        &self,
    ) -> Result<Vec<Cluster>, ClusterRepositoryError> {
        self.repo.list_clusters().await
    }

    async fn delete_cluster(
        &self,
        id: &ClusterId,
    ) -> Result<(), ClusterRepositoryError> {
        self.repo.delete_cluster(id).await
    }
}
