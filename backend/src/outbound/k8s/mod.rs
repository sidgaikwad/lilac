pub mod factory;
use std::time::Duration;

use hyper_util::rt::TokioExecutor;
use k8s_openapi::api::core::v1::Pod;
use kube::{api::ListParams, client::ConfigExt, Api, Client, Config};
use tower::ServiceBuilder;
use tower_http::BoxError;

pub const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(30);
pub const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(295);
pub const DEFAULT_WRITE_TIMEOUT: Duration = Duration::from_secs(295);

#[derive(Clone)]
pub struct K8sManager {
    client: kube::Client,
}

impl K8sManager {
    pub fn new(kube_config: Config) -> anyhow::Result<Self> {
        let https = kube_config.rustls_https_connector()?;
        let service = ServiceBuilder::new()
            .layer(kube_config.base_uri_layer())
            .option_layer(kube_config.auth_layer()?)
            .map_err(BoxError::from)
            .service(
                hyper_util::client::legacy::Client::builder(TokioExecutor::new()).build(https),
            );
        Ok(Self {
            client: Client::new(service, kube_config.default_namespace),
        })
    }

    pub async fn list_pods(&self) -> anyhow::Result<Vec<Pod>> {
        let api: Api<Pod> = Api::default_namespaced(self.client.clone());
        let resp = api.list(&ListParams::default()).await?;
        Ok(resp.items)
    }
}

#[cfg(test)]
mod tests {
    use k8s_openapi::api::core::v1::Pod;
    use kube::{api::ListParams, Api, Config};

    use crate::outbound::k8s::K8sManager;

    #[test_log::test(tokio::test)]
    async fn test_k8s_manager() {
        rustls::crypto::ring::default_provider()
            .install_default()
            .unwrap();
        let mut mock_server = mockito::Server::new_async().await;
        let mock = mock_server
            .mock("GET", "/api/v1/namespaces/lilac/pods?")
            .with_status(200)
            .with_body(
                r#"{
                "apiVersion": "v1",
                "items": [
                    {
                        "apiVersion": "v1",
                        "kind": "Pod",
                        "metadata": {
                            "creationTimestamp": "2025-01-01T12:34:56Z",
                            "name": "test-pod-1",
                            "namespace": "lilac"
                        }
                    },
                    {
                        "apiVersion": "v1",
                        "kind": "Pod",
                        "metadata": {
                            "creationTimestamp": "2025-01-01T12:34:56Z",
                            "name": "test-pod-2",
                            "namespace": "lilac"
                        }
                    }
                ],
                "kind": "List",
                "metadata": {
                    "resourceVersion": ""
                }
            }"#,
            )
            .create();
        let k8s = K8sManager::new(Config::new(mock_server.url().parse().unwrap())).unwrap();

        let api: Api<Pod> = Api::namespaced(k8s.client, "lilac");
        let resp = api.list(&ListParams::default()).await;
        mock.assert();
        assert!(resp.is_ok());
        let pods = resp.unwrap().items;
        assert_eq!(pods.len(), 2);
        assert_eq!(pods[0].metadata.name.as_ref().unwrap(), "test-pod-1");
        assert_eq!(pods[1].metadata.name.as_ref().unwrap(), "test-pod-2");
    }
}
