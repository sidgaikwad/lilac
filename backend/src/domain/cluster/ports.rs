use async_trait::async_trait;
use thiserror::Error;

use super::models::{Cluster, ClusterId, CreateClusterRequest};

#[derive(Debug, Error)]
pub enum ClusterRepositoryError {
    #[error("cluster with {field} {value} already exists")]
    Duplicate { field: String, value: String },
    #[error("cluster with id {0} not found")]
    NotFound(String),
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
    async fn list_clusters(&self) -> Result<Vec<Cluster>, ClusterRepositoryError>;
    async fn delete_cluster(&self, id: &ClusterId) -> Result<(), ClusterRepositoryError>;
}
