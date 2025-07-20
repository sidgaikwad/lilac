use crate::{
    domain::{
        cluster::{
            models::ClusterConfig,
            ports::{ClusterConnectionError, ClusterConnectionTester},
        },
        credentials::models::Credentials,
    },
    outbound::{aws::AwsEksAdapter, gcp::GkeAdapter, k8s::K8sManager},
};
use async_trait::async_trait;

#[derive(Clone)]
pub struct ClusterConnectorImpl {}

impl ClusterConnectorImpl {
    pub fn new() -> Self {
        Self {}
    }

    async fn test_eks_connection(
        &self,
        cluster_name: String,
        region: String,
        credentials: Credentials,
    ) -> Result<(), ClusterConnectionError> {
        let kube_config = match credentials {
            Credentials::Aws {
                access_key,
                secret_key,
            } => {
                let aws = AwsEksAdapter::new(access_key, secret_key, Some(region.to_string()));
                aws.get_eks_kube_config(&cluster_name).await?
            }
            _ => {
                return Err(ClusterConnectionError::InvalidCredentials(
                    "incorrect credential type".into(),
                ))
            }
        };
        let k8s = K8sManager::new(kube_config)?;
        k8s.list_pods().await?;
        Ok(())
    }

    async fn test_gke_connection(
        &self,
        project_id: String,
        cluster_name: String,
        region: String,
        credentials: Credentials,
    ) -> Result<(), ClusterConnectionError> {
        let kube_config = match credentials {
            Credentials::Gcp(credentials) => {
                let gke = GkeAdapter::new(credentials).await?;
                gke.get_gke_kube_config(&project_id, &region, &cluster_name)
                    .await?
            }
            _ => {
                return Err(ClusterConnectionError::InvalidCredentials(
                    "incorrect credential type".into(),
                ))
            }
        };
        let k8s = K8sManager::new(kube_config)?;
        k8s.list_pods().await?;
        Ok(())
    }
}

#[async_trait]
impl ClusterConnectionTester for ClusterConnectorImpl {
    async fn test_cluster_connection(
        &self,
        credentials: Credentials,
        cluster_config: ClusterConfig,
    ) -> Result<(), ClusterConnectionError> {
        match cluster_config {
            ClusterConfig::Local => {
                // No external connection to test for a local cluster
            }
            ClusterConfig::AwsEks {
                cluster_name,
                region,
            } => {
                self.test_eks_connection(cluster_name, region, credentials)
                    .await?
            }
            ClusterConfig::GcpGke {
                project_id,
                cluster_name,
                location,
            } => {
                self.test_gke_connection(project_id, cluster_name, location, credentials)
                    .await?
            }
        }
        Ok(())
    }
}
