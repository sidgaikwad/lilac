use async_trait::async_trait;

use crate::{
    domain::{
        cluster::{
            models::ClusterConfig,
            ports::{ClusterConnectionError, ClusterConnectionTester},
        },
        credentials::models::Credentials,
    },
    outbound::{aws::AwsEksAdapter, k8s::K8sManager},
};

#[derive(Clone)]
pub struct ClusterConnectorImpl {}

impl ClusterConnectorImpl {
    pub fn new() -> Self {
        Self {}
    }

    async fn test_kubernetes_connection(
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
            ClusterConfig::AwsEks {
                cluster_name,
                region,
            } => {
                self.test_kubernetes_connection(cluster_name, region, credentials)
                    .await?
            }
        }
        Ok(())
    }
}
