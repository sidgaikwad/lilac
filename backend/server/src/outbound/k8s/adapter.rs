use std::fmt::Debug;

use k8s_openapi::api::{core::v1::Namespace, rbac::v1::Role};
use kube::{
    api::{DeleteParams, ListParams, ObjectMeta, PostParams},
    Api,
};
use kube::{Client, Config};

use crate::domain::integration::ports::K8sPort;

#[derive(Clone)]
pub struct K8sAdapter {
    client: Client,
}

impl Debug for K8sAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("K8sAdapter").finish()
    }
}

impl K8sAdapter {
    pub async fn new() -> Self {
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

use crate::domain::integration::ports::K8sPortError;
use async_trait::async_trait;
use std::str;

use super::helm;

#[async_trait]
impl K8sPort for K8sAdapter {
    async fn create_namespace(&self, namespace: &str) -> Result<(), K8sPortError> {
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

    async fn delete_namespace(&self, namespace: &str) -> Result<(), K8sPortError> {
        let client: Api<Namespace> = Api::all(self.client.clone());
        client.delete(namespace, &DeleteParams::default()).await?;
        Ok(())
    }

    async fn list_namespaces(&self) -> Result<Vec<String>, K8sPortError> {
        let client: Api<Namespace> = Api::all(self.client.clone());
        let namespaces = client.list(&ListParams::default()).await?;
        Ok(namespaces
            .items
            .into_iter()
            .filter_map(|v| v.metadata.name)
            .collect())
    }

    async fn create_role(&self, namespace: &str, role: Role) -> Result<(), K8sPortError> {
        let client: Api<Role> = Api::namespaced(self.client.clone(), namespace);
        client.create(&PostParams::default(), &role).await?;
        Ok(())
    }

    async fn delete_role(&self, _namespace: &str, _role_name: &str) -> Result<(), K8sPortError> {
        todo!()
    }

    async fn list_roles(&self, _namespace: &str) -> Result<Vec<String>, K8sPortError> {
        todo!()
    }

    async fn create_role_binding(
        &self,
        _namespace: &str,
        _role_name: &str,
        _user: &str,
    ) -> Result<(), K8sPortError> {
        todo!()
    }

    async fn delete_role_binding(
        &self,
        _namespace: &str,
        _role_name: &str,
        _user: &str,
    ) -> Result<(), K8sPortError> {
        todo!()
    }

    async fn list_role_bindings(
        &self,
        _namespace: &str,
        _user: &str,
    ) -> Result<Vec<String>, K8sPortError> {
        todo!()
    }
    
    async fn helm_install(
        &self,
        namespace: &str,
        name: &str,
        chart: &str,
        values: Option<Vec<&str>>,
    ) -> Result<(), K8sPortError> {
        helm::helm_install(namespace, name, chart, values).await
    }

    async fn helm_uninstall(&self, namespace: &str, name: &str) -> Result<(), K8sPortError> {
        helm::helm_uninstall(namespace, name).await
    }
}