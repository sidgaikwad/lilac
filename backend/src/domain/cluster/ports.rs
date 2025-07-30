use async_trait::async_trait;
use thiserror::Error;

use crate::domain::{
    cluster::models::{ClusterDetails, ClusterNode, NodeId, UpdateNodeStatusRequest},
    training_job::models::{JobId, TrainingJob},
    user::models::{ApiKey, ApiKeyId}
};

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
    async fn get_cluster_details(&self, id: &ClusterId) -> Result<ClusterDetails, ClusterRepositoryError>;
    async fn list_clusters(&self) -> Result<Vec<Cluster>, ClusterRepositoryError>;
    async fn delete_cluster(&self, id: &ClusterId) -> Result<(), ClusterRepositoryError>;
    async fn list_cluster_jobs(
        &self,
        id: &ClusterId,
    ) -> Result<Vec<TrainingJob>, ClusterRepositoryError>;
    async fn list_all_nodes(&self) -> Result<Vec<ClusterNode>, ClusterRepositoryError>;
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
    async fn clear_assigned_job_id(&self, node_id: &NodeId) -> Result<(), ClusterRepositoryError>;
    async fn assign_job_to_node(
        &self,
        node_id: &NodeId,
        job_id: &JobId,
    ) -> Result<(), ClusterRepositoryError>;
}

#[derive(Debug, Error)]
pub enum ClusterApiKeyRepositoryError {
    #[error("api key not found")]
    NotFound,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[async_trait]
pub trait ClusterApiKeyRepository: Send + Sync + 'static {
    async fn create_api_key(&self, key: &ApiKey) -> Result<(), ClusterApiKeyRepositoryError>;
    async fn find_cluster_by_api_key_hash(
        &self,
        key_hash: &str,
    ) -> Result<Cluster, ClusterApiKeyRepositoryError>;
    async fn list_api_keys_for_cluster(
        &self,
        cluster_id: &ClusterId,
    ) -> Result<Vec<ApiKey>, ClusterApiKeyRepositoryError>;
    async fn delete_api_key(&self, id: &ApiKeyId) -> Result<(), ClusterApiKeyRepositoryError>;
}
