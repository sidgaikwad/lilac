use async_trait::async_trait;
use thiserror::Error;

use crate::domain::credentials::models::Credentials;

use super::models::{Cluster, ClusterConfig, ClusterId, CreateClusterRequest};

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

#[derive(Debug, Error)]
pub enum ClusterConnectionError {
    #[error("invalid credentials: {0}")]
    InvalidCredentials(String),
    #[error("could not reach cluster: {0}")]
    ClusterUnreachable(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[async_trait]
pub trait ClusterConnectionTester: Send + Sync {
    async fn test_cluster_connection(
        &self,
        credentials: Credentials,
        cluster_config: ClusterConfig,
    ) -> Result<(), ClusterConnectionError>;
}
