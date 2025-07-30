use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::{
    cluster::models::{ClusterDetails, ClusterNode, UpdateNodeStatusRequest},
    training_job::models::TrainingJob,
    user::models::{ApiKey, ApiKeyId}
};

use crate::domain::user::models::NewApiKey;
use secrecy::SecretString;

use chrono::Utc;
use secrecy::ExposeSecret;
use sha2::{Digest, Sha256};

const API_KEY_PREFIX: &str = "lilac_sk_";
const NANOID_ALPHABET: [char; 62] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

use super::{
    models::{Cluster, ClusterId, CreateClusterRequest},
    ports::{
        ClusterApiKeyRepository, ClusterApiKeyRepositoryError, ClusterRepository,
        ClusterRepositoryError,
    },
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

impl From<ClusterApiKeyRepositoryError> for ClusterServiceError {
    fn from(error: ClusterApiKeyRepositoryError) -> Self {
        match error {
            ClusterApiKeyRepositoryError::NotFound => Self::ClusterNotFound("".to_string()), // TODO: Better error
            ClusterApiKeyRepositoryError::Unknown(error) => Self::Unknown(error),
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
    async fn get_cluster_details(
        &self,
        cluster_id: &ClusterId,
    ) -> Result<ClusterDetails, ClusterServiceError>;
    async fn list_clusters(&self) -> Result<Vec<Cluster>, ClusterServiceError>;
    async fn delete_cluster(&self, cluster_id: &ClusterId) -> Result<(), ClusterServiceError>;
    async fn list_cluster_jobs(
        &self,
        cluster_id: &ClusterId,
    ) -> Result<Vec<TrainingJob>, ClusterServiceError>;
    async fn list_cluster_nodes(
        &self,
        cluster_id: &ClusterId,
    ) -> Result<Vec<ClusterNode>, ClusterServiceError>;
    async fn update_node_status(
        &self,
        req: UpdateNodeStatusRequest,
    ) -> Result<ClusterNode, ClusterServiceError>;
    async fn authenticate_by_api_key(
        &self,
        key: &SecretString,
    ) -> Result<Cluster, ClusterServiceError>;
    async fn create_api_key_for_cluster(
        &self,
        cluster_id: &ClusterId,
    ) -> Result<NewApiKey, ClusterServiceError>;
    async fn list_api_keys(
        &self,
        cluster_id: &ClusterId,
    ) -> Result<Vec<ApiKey>, ClusterServiceError>;
    async fn clear_assigned_job_id(&self, node_id: &super::models::NodeId) -> Result<(), ClusterServiceError>;
}

#[derive(Clone)]
pub struct ClusterServiceImpl<R: ClusterRepository + ClusterApiKeyRepository> {
    cluster_repo: Arc<R>,
}

impl<R: ClusterRepository + ClusterApiKeyRepository> ClusterServiceImpl<R> {
    pub fn new(cluster_repo: Arc<R>) -> Self {
        Self { cluster_repo }
    }
}

#[async_trait]
impl<R: ClusterRepository + ClusterApiKeyRepository> ClusterService for ClusterServiceImpl<R> {
    async fn create_cluster(
        &self,
        req: &CreateClusterRequest,
    ) -> Result<Cluster, ClusterServiceError> {
        let cluster = self.cluster_repo.create_cluster(req).await?;
        Ok(cluster)
    }

    async fn get_cluster_by_id(
        &self,
        cluster_id: &ClusterId,
    ) -> Result<Cluster, ClusterServiceError> {
        Ok(self.cluster_repo.get_cluster_by_id(cluster_id).await?)
    }

    async fn get_cluster_details(
        &self,
        cluster_id: &ClusterId,
    ) -> Result<ClusterDetails, ClusterServiceError> {
        Ok(self.cluster_repo.get_cluster_details(cluster_id).await?)
    }

    async fn list_clusters(&self) -> Result<Vec<Cluster>, ClusterServiceError> {
        Ok(self.cluster_repo.list_clusters().await?)
    }

    async fn delete_cluster(&self, cluster_id: &ClusterId) -> Result<(), ClusterServiceError> {
        Ok(self.cluster_repo.delete_cluster(cluster_id).await?)
    }

    async fn list_cluster_jobs(
        &self,
        cluster_id: &ClusterId,
    ) -> Result<Vec<TrainingJob>, ClusterServiceError> {
        Ok(self.cluster_repo.list_cluster_jobs(cluster_id).await?)
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

    async fn authenticate_by_api_key(
        &self,
        key: &SecretString,
    ) -> Result<Cluster, ClusterServiceError> {
        let mut hasher = Sha256::new();
        hasher.update(key.expose_secret().as_bytes());
        let key_hash = format!("{:x}", hasher.finalize());

        let cluster = self.cluster_repo.find_cluster_by_api_key_hash(&key_hash).await?;

        Ok(cluster)
    }

    async fn create_api_key_for_cluster(
        &self,
        cluster_id: &ClusterId,
    ) -> Result<NewApiKey, ClusterServiceError> {
        self.cluster_repo.get_cluster_by_id(cluster_id).await?;

        let key_id = ApiKeyId::generate();
        let raw_key = nanoid::nanoid!(32, &NANOID_ALPHABET);
        let secret_key = SecretString::from(raw_key);
        let full_key = format!("{}{}", API_KEY_PREFIX, secret_key.expose_secret());

        let mut hasher = Sha256::new();
        hasher.update(full_key.as_bytes());
        let key_hash = format!("{:x}", hasher.finalize());

        let api_key = ApiKey {
            id: key_id,
            user_id: None,
            cluster_id: Some(cluster_id.clone()),
            prefix: API_KEY_PREFIX.to_string(),
            key_hash,
            created_at: Utc::now(),
            last_used_at: None,
            expires_at: None,
        };

        self.cluster_repo
            .create_api_key(&api_key)
            .await
            .map_err(|e| ClusterServiceError::Unknown(e.into()))?;

        let new_api_key = NewApiKey {
            id: key_id,
            prefix: API_KEY_PREFIX.to_string(),
            key: SecretString::from(full_key),
            created_at: api_key.created_at,
        };

        Ok(new_api_key)
    }

    async fn list_api_keys(
        &self,
        cluster_id: &ClusterId,
    ) -> Result<Vec<ApiKey>, ClusterServiceError> {
        Ok(self
            .cluster_repo
            .list_api_keys_for_cluster(cluster_id)
            .await?)
    }
    async fn clear_assigned_job_id(&self, node_id: &super::models::NodeId) -> Result<(), ClusterServiceError> {
        self.cluster_repo.clear_assigned_job_id(node_id).await?;
        Ok(())
    }
}
