use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;

use crate::domain::{
    cluster::{models::ClusterConfig, ports::ClusterConnectionTester},
    credentials::{
        models::{CredentialId, Credentials},
        ports::{CredentialRepository, CredentialRepositoryError},
    },
};

use super::{
    models::{Cluster, ClusterId, CreateClusterRequest},
    ports::{ClusterRepository, ClusterRepositoryError},
};

#[derive(Debug, thiserror::Error)]
pub enum ClusterServiceError {
    #[error("invalid permissions")]
    InvalidPermissions,
    #[error("incorrect credentials type")]
    IncorrectCredentialsType,
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

impl From<CredentialRepositoryError> for ClusterServiceError {
    fn from(error: CredentialRepositoryError) -> Self {
        match error {
            CredentialRepositoryError::Duplicate { .. } => {
                Self::Unknown(anyhow!("duplicate credential"))
            }
            CredentialRepositoryError::NotFound(id) => Self::ClusterNotFound(id),
            CredentialRepositoryError::Unknown(error) => Self::Unknown(error),
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
    async fn test_cluster_connection(
        &self,
        credential_id: CredentialId,
        cluster_config: ClusterConfig,
    ) -> Result<(), ClusterServiceError>;
}

#[derive(Clone)]
pub struct ClusterServiceImpl<
    R: ClusterRepository,
    C: CredentialRepository,
    T: ClusterConnectionTester,
> {
    cluster_repo: Arc<R>,
    credential_repo: Arc<C>,
    conn_tester: Arc<T>,
}

impl<R: ClusterRepository, C: CredentialRepository, T: ClusterConnectionTester>
    ClusterServiceImpl<R, C, T>
{
    pub fn new(cluster_repo: Arc<R>, credential_repo: Arc<C>, conn_tester: Arc<T>) -> Self {
        Self {
            cluster_repo,
            credential_repo,
            conn_tester,
        }
    }
}

#[async_trait]
impl<R: ClusterRepository, C: CredentialRepository, T: ClusterConnectionTester> ClusterService
    for ClusterServiceImpl<R, C, T>
{
    async fn create_cluster(
        &self,
        req: &CreateClusterRequest,
    ) -> Result<Cluster, ClusterServiceError> {
        let credentials = self
            .credential_repo
            .get_credential_by_id(&req.credential_id)
            .await?;
        match req.clone().cluster_config {
            ClusterConfig::AwsEks { .. } => {
                match credentials.credentials {
                    Credentials::Aws { .. } => Ok(self.cluster_repo.create_cluster(req).await?), // _ => Err(ClusterServiceError::IncorrectCredentialsType)
                }
            }
        }
    }

    async fn get_cluster_by_id(&self, id: &ClusterId) -> Result<Cluster, ClusterServiceError> {
        Ok(self.cluster_repo.get_cluster_by_id(id).await?)
    }

    async fn list_clusters(&self) -> Result<Vec<Cluster>, ClusterServiceError> {
        Ok(self.cluster_repo.list_clusters().await?)
    }

    async fn delete_cluster(&self, id: &ClusterId) -> Result<(), ClusterServiceError> {
        Ok(self.cluster_repo.delete_cluster(id).await?)
    }

    async fn test_cluster_connection(
        &self,
        credential_id: CredentialId,
        cluster_config: ClusterConfig,
    ) -> Result<(), ClusterServiceError> {
        let credential = self
            .credential_repo
            .get_credential_by_id(&credential_id)
            .await?;
        let _ = self
            .conn_tester
            .test_cluster_connection(credential.credentials, cluster_config)
            .await;
        Ok(())
    }
}
