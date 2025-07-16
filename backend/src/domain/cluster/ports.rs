use async_trait::async_trait;
use thiserror::Error;

use super::models::{CreateClusterRequest, Cluster, ClusterId};

#[derive(Debug, Error)]
pub enum ClusterRepositoryError {
    #[error("{0} with value {1} already exists")]
    Duplicate(String, String),
    #[error("cluster with id {0} not found")]
    NotFound(String),
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[async_trait]
pub trait ClusterRepository: Send + Sync {
    async fn create_cluster(
        &self,
        req: &CreateClusterRequest,
    ) -> Result<Cluster, ClusterRepositoryError>;
    async fn get_cluster_by_id(&self, id: &ClusterId) -> Result<Cluster, ClusterRepositoryError>;
    async fn list_clusters(
        &self,
    ) -> Result<Vec<Cluster>, ClusterRepositoryError>;
    async fn delete_cluster(&self, id: &ClusterId) -> Result<(), ClusterRepositoryError>;
}
