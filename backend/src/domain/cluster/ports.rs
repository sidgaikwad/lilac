use async_trait::async_trait;
use thiserror::Error;

use crate::domain::cluster::models::{ClusterNode, NodeId, UpdateNodeStatusRequest};

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

// Add assign job id fn?

#[async_trait]
pub trait ClusterRepository: Send + Sync {
    async fn create_cluster(
        &self,
        req: &CreateClusterRequest,
    ) -> Result<Cluster, ClusterRepositoryError>;
    async fn get_cluster_by_id(&self, id: &ClusterId) -> Result<Cluster, ClusterRepositoryError>;
    async fn list_clusters(&self) -> Result<Vec<Cluster>, ClusterRepositoryError>;
    async fn delete_cluster(&self, id: &ClusterId) -> Result<(), ClusterRepositoryError>;
    async fn list_cluster_nodes(
        &self,
        id: &ClusterId,
    ) -> Result<Vec<ClusterNode>, ClusterRepositoryError>;
    async fn get_cluster_node_by_id(
        &self,
        id: &NodeId,
    ) -> Result<ClusterNode, ClusterRepositoryError>;
    async fn update_cluster_node_status(
        &self,
        req: &UpdateNodeStatusRequest,
    ) -> Result<ClusterNode, ClusterRepositoryError>;
    async fn delete_cluster_node(&self, node_id: &NodeId) -> Result<(), ClusterRepositoryError>;
}
