use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::cluster::models::{ClusterNode, UpdateNodeStatusRequest};

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
// Add error for assign job id
#[async_trait]
pub trait ClusterService: Send + Sync {
    async fn create_cluster(
        &self,
        req: &CreateClusterRequest,
    ) -> Result<Cluster, ClusterServiceError>;
    async fn get_cluster_by_id(
        &self,
        cluster_id: &ClusterId,
    ) -> Result<Cluster, ClusterServiceError>;
    async fn list_clusters(&self) -> Result<Vec<Cluster>, ClusterServiceError>;
    async fn delete_cluster(&self, cluster_id: &ClusterId) -> Result<(), ClusterServiceError>;
    async fn list_cluster_nodes(
        &self,
        cluster_id: &ClusterId,
    ) -> Result<Vec<ClusterNode>, ClusterServiceError>;
    async fn update_node_status(
        &self,
        req: UpdateNodeStatusRequest,
    ) -> Result<ClusterNode, ClusterServiceError>;
}

#[derive(Clone)]
pub struct ClusterServiceImpl<R: ClusterRepository> {
    cluster_repo: Arc<R>,
}

impl<R: ClusterRepository> ClusterServiceImpl<R> {
    pub fn new(cluster_repo: Arc<R>) -> Self {
        Self { cluster_repo }
    }
}

#[async_trait]
impl<R: ClusterRepository> ClusterService for ClusterServiceImpl<R> {
    async fn create_cluster(
        &self,
        req: &CreateClusterRequest,
    ) -> Result<Cluster, ClusterServiceError> {
        // TODO: After creating the cluster, also create a new ApiKey owned by this cluster.
        Ok(self.cluster_repo.create_cluster(req).await?)
    }

    async fn get_cluster_by_id(
        &self,
        cluster_id: &ClusterId,
    ) -> Result<Cluster, ClusterServiceError> {
        Ok(self.cluster_repo.get_cluster_by_id(cluster_id).await?)
    }

    async fn list_clusters(&self) -> Result<Vec<Cluster>, ClusterServiceError> {
        Ok(self.cluster_repo.list_clusters().await?)
    }

    async fn delete_cluster(&self, cluster_id: &ClusterId) -> Result<(), ClusterServiceError> {
        Ok(self.cluster_repo.delete_cluster(cluster_id).await?)
    }

    async fn update_node_status(
        &self,
        req: UpdateNodeStatusRequest,
    ) -> Result<ClusterNode, ClusterServiceError> {
        let node = self.cluster_repo.update_cluster_node_status(&req).await?;

        if node.assigned_job_id != node.reported_job_id {
            // TODO: Implement reconciliation logic.
            // This could involve logging a warning, emitting an event,
            // or telling the agent to terminate the unexpected job.
            // For now, we will just log it.
            tracing::warn!(
                node_id = %node.id,
                assigned_job_id = ?node.assigned_job_id,
                reported_job_id = ?node.reported_job_id,
                "Mismatched job ID reported by agent."
            );
        }

        Ok(node)
    }

    async fn list_cluster_nodes(
        &self,
        cluster_id: &ClusterId,
    ) -> Result<Vec<ClusterNode>, ClusterServiceError> {
        Ok(self.cluster_repo.list_cluster_nodes(cluster_id).await?)
    }
}
