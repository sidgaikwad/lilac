use std::{fmt::Debug, future::Future};

use k8s_openapi::api::{
    core::v1::Namespace,
    rbac::v1::Role,
};
use kube::{
    api::{DeleteParams, ListParams, ObjectMeta, PostParams},
    Api,
};
use kube::{Client, Config};

pub mod helm;
pub mod policies;

#[derive(Clone)]
pub struct K8sWrapper {
    client: Client,
}

impl Debug for K8sWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("K8sWrapper").finish()
    }
}

impl K8sWrapper {
    pub async fn new(_url: String) -> Self {
        let client = Client::try_from(Config::infer().await.unwrap()).unwrap();
        Self { client }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum K8sError {
    #[error("{0}")]
    Kubernetes(#[from] kube::Error),

    #[error("{0}")]
    Helm(String),
}

pub trait K8sApi {
    fn create_namespace(
        &self,
        namespace: &str,
    ) -> impl Future<Output = Result<(), K8sError>> + Send;

    fn delete_namespace(
        &self,
        namespace: &str,
    ) -> impl Future<Output = Result<(), K8sError>> + Send;

    fn list_namespaces(&self) -> impl Future<Output = Result<Vec<String>, K8sError>> + Send;

    fn create_role(
        &self,
        namespace: &str,
        role: Role,
    ) -> impl Future<Output = Result<(), K8sError>> + Send;

    fn delete_role(
        &self,
        namespace: &str,
        role_name: &str,
    ) -> impl Future<Output = Result<(), K8sError>> + Send;

    fn list_roles(
        &self,
        namespace: &str,
    ) -> impl Future<Output = Result<Vec<String>, K8sError>> + Send;

    fn create_role_binding(
        &self,
        namespace: &str,
        role_name: &str,
        user: &str,
    ) -> impl Future<Output = Result<(), K8sError>> + Send;

    fn delete_role_binding(
        &self,
        namespace: &str,
        role_name: &str,
        user: &str,
    ) -> impl Future<Output = Result<(), K8sError>> + Send;

    fn list_role_bindings(
        &self,
        namespace: &str,
        user: &str,
    ) -> impl Future<Output = Result<Vec<String>, K8sError>> + Send;
}

impl K8sApi for K8sWrapper {
    async fn create_namespace(&self, namespace: &str) -> Result<(), super::K8sError> {
        let client: Api<Namespace> = Api::all(self.client.clone());
        client
            .create(
                &PostParams::default(),
                &Namespace {
                    metadata: ObjectMeta {
                        name: Some(namespace.to_string()),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
            .await?;
        Ok(())
    }

    async fn delete_namespace(&self, namespace: &str) -> Result<(), super::K8sError> {
        let client: Api<Namespace> = Api::all(self.client.clone());
        client.delete(namespace, &DeleteParams::default()).await?;
        Ok(())
    }

    async fn list_namespaces(&self) -> Result<Vec<String>, super::K8sError> {
        let client: Api<Namespace> = Api::all(self.client.clone());
        let namespaces = client.list(&ListParams::default()).await?;
        Ok(namespaces
            .items
            .into_iter()
            .filter_map(|v| v.metadata.name)
            .collect())
    }

    async fn create_role(&self, namespace: &str, role: Role) -> Result<(), K8sError> {
        let client: Api<Role> = Api::namespaced(self.client.clone(), namespace);
        client.create(&PostParams::default(), &role).await?;
        Ok(())
    }

    async fn delete_role(&self, namespace: &str, role_name: &str) -> Result<(), K8sError> {
        todo!()
    }

    async fn list_roles(&self, namespace: &str) -> Result<Vec<String>, K8sError> {
        todo!()
    }

    async fn create_role_binding(
        &self,
        namespace: &str,
        role_name: &str,
        user: &str,
    ) -> Result<(), K8sError> {
        todo!()
    }

    async fn delete_role_binding(
        &self,
        namespace: &str,
        role_name: &str,
        user: &str,
    ) -> Result<(), K8sError> {
        todo!()
    }

    async fn list_role_bindings(
        &self,
        namespace: &str,
        user: &str,
    ) -> Result<Vec<String>, K8sError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Once;

    use kube::{Client, Config};
    use rustls::crypto::ring::default_provider;

    use crate::k8s::{K8sApi, K8sWrapper};

    static INIT_CRYPTO: Once = Once::new();

    fn create_client(url: String) -> K8sWrapper {
        INIT_CRYPTO.call_once(|| default_provider().install_default().unwrap());
        K8sWrapper {
            client: Client::try_from(Config::new(url.try_into().unwrap())).unwrap(),
        }
    }

    #[test_log::test(tokio::test)]
    async fn test_create_namespace() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        println!("{url}");

        let mock = server
            .mock("POST", "/api/v1/namespaces?")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "apiVersion": "v1",
                "kind": "Namespace",
                "metadata": {
                    "name": "test-namespace"
                }
            }"#,
            )
            .create();

        let client = create_client(url);
        let res = client.create_namespace("test-namespace").await;
        assert!(res.is_ok(), "{res:?}");
        mock.assert();
    }

    #[test_log::test(tokio::test)]
    async fn test_delete_namespace() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock_create = server
            .mock("POST", "/api/v1/namespaces?")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "apiVersion": "v1",
                "kind": "Namespace",
                "metadata": {
                    "name": "test-namespace"
                }
            }"#,
            )
            .create();
        let mock_delete = server
            .mock("DELETE", "/api/v1/namespaces/test-namespace?")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "apiVersion": "v1",
                "kind": "Namespace",
                "metadata": {
                    "name": "test-namespace"
                }
            }"#,
            )
            .create();

        let client = create_client(url);

        let res = client.create_namespace("test-namespace").await;
        assert!(res.is_ok(), "{res:?}");
        mock_create.assert();

        let res = client.delete_namespace("test-namespace").await;
        assert!(res.is_ok(), "{res:?}");
        mock_delete.assert();
    }

    #[test_log::test(tokio::test)]
    async fn test_list_namespaces() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        println!("{url}");

        let mock = server
            .mock("GET", "/api/v1/namespaces?")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "apiVersion": "v1",
                "kind": "NamespaceList",
                "items": [
                    {
                        "apiVersion": "v1",
                        "kind": "Namespace",
                        "metadata": {
                            "name": "test-namespace"
                        }
                    }
                ],
                "metadata": {
                    "name": "test-namespace"
                }
            }"#,
            )
            .create();

        let client = create_client(url);
        let res = client.list_namespaces().await;
        assert!(res.is_ok(), "{res:?}");
        mock.assert();
        let namespaces = res.unwrap();
        assert_eq!(namespaces, vec!["test-namespace"]);
    }
}
