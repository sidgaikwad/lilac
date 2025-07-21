use std::sync::Arc;
use kube::Client;
use thiserror::Error;

use crate::{
    domain::{
        cluster::models::{Cluster, ClusterConfig},
        credentials::{
            models::Credentials,
            ports::{CredentialRepository, CredentialRepositoryError},
        },
    },
    outbound::aws::AwsEksAdapter,
};
use hyper_util::rt::TokioExecutor;
use kube::{client::ConfigExt, Config};
use tower_http::BoxError;

#[derive(Error, Debug)]
pub enum KubeClientFactoryError {
    #[error("Cluster not found")]
    ClusterNotFound,
    #[error("Failed to create kube client: {0}")]
    ClientCreation(#[from] anyhow::Error),
    #[error("Unsupported cluster type")]
    UnsupportedClusterType,
    #[error("Credential error: {0}")]
    CredentialError(String),
}

impl From<CredentialRepositoryError> for KubeClientFactoryError {
    fn from(value: CredentialRepositoryError) -> Self {
        match value {
            CredentialRepositoryError::NotFound(id) => Self::CredentialError(format!("not found: {}", id)),
            _ => Self::CredentialError("unknown error".to_string()),
        }
    }
}

#[async_trait::async_trait]
pub trait KubeClientFactory: Send + Sync {
    async fn create_client(&self, cluster: &Cluster) -> Result<Client, KubeClientFactoryError>;
}

pub struct KubeClientFactoryImpl {
    credential_repository: Arc<dyn CredentialRepository>,
}

impl KubeClientFactoryImpl {
    pub fn new(credential_repository: Arc<dyn CredentialRepository>) -> Self {
        Self {
            credential_repository,
        }
    }
}

#[async_trait::async_trait]
impl KubeClientFactory for KubeClientFactoryImpl {
    async fn create_client(&self, cluster: &Cluster) -> Result<Client, KubeClientFactoryError> {
        let kube_config = match &cluster.cluster_config {
            ClusterConfig::Local => Config::infer()
                .await
                .map_err(|e| KubeClientFactoryError::ClientCreation(e.into()))?,
            ClusterConfig::AwsEks {
                cluster_name,
                region,
            } => {
                let credential = self
                    .credential_repository
                    .get_credential_by_id(&cluster.credential_id)
                    .await?;

                match credential.credentials {
                    Credentials::Aws {
                        access_key,
                        secret_key,
                    } => {
                        let aws_adapter =
                            AwsEksAdapter::new(access_key, secret_key, Some(region.clone()));
                        aws_adapter.get_eks_kube_config(cluster_name).await?
                    }
                }
            }
        };

        let https = kube_config
            .rustls_https_connector()
            .map_err(|e| KubeClientFactoryError::ClientCreation(e.into()))?;
        let service = tower::ServiceBuilder::new()
            .layer(kube_config.base_uri_layer())
            .option_layer(
                kube_config
                    .auth_layer()
                    .map_err(|e| KubeClientFactoryError::ClientCreation(e.into()))?,
            )
            .map_err(BoxError::from)
            .service(hyper_util::client::legacy::Client::builder(TokioExecutor::new()).build(https));
        Ok(Client::new(
            service,
            kube_config.default_namespace.clone(),
        ))
    }
}