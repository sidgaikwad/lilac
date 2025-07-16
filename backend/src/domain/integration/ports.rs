use async_trait::async_trait;
use thiserror::Error;

use super::models::{AWSIntegration, CreateAWSIntegrationRequest, Integration, IntegrationId};
use crate::domain::project::models::ProjectId;

#[derive(Debug, Error)]
pub enum IntegrationRepositoryError {
    #[error("integration with id {0} already exists")]
    Duplicate(String),
    #[error("integration with id {0} not found")]
    NotFound(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

use crate::domain::user::models::UserId;

#[async_trait]
pub trait IntegrationRepository: Clone + Send + Sync + 'static {
    async fn create_aws_integration(
        &self,
        user_id: &UserId,
        req: &CreateAWSIntegrationRequest,
    ) -> Result<AWSIntegration, IntegrationRepositoryError>;

    async fn get_integrations_by_project_id(
        &self,
        user_id: &UserId,
        project_id: &ProjectId,
    ) -> Result<Vec<Integration>, IntegrationRepositoryError>;

    async fn get_integration_by_id(
        &self,
        user_id: &UserId,
        integration_id: &IntegrationId,
    ) -> Result<Integration, IntegrationRepositoryError>;
}
#[derive(Debug, Error)]
pub enum K8sPortError {
    #[error("helm command failed: {0}")]
    Helm(String),
    #[error(transparent)]
    Kube(#[from] kube::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[async_trait]
pub trait K8sPort: Clone + Send + Sync + 'static {
    async fn helm_install(
        &self,
        namespace: &str,
        name: &str,
        chart: &str,
        values: Option<Vec<&str>>,
    ) -> Result<(), K8sPortError>;

    async fn helm_uninstall(&self, namespace: &str, name: &str) -> Result<(), K8sPortError>;
    async fn create_namespace(&self, namespace: &str) -> Result<(), K8sPortError>;
    async fn delete_namespace(&self, namespace: &str) -> Result<(), K8sPortError>;
    async fn list_namespaces(&self) -> Result<Vec<String>, K8sPortError>;
    async fn create_role(
        &self,
        namespace: &str,
        role: k8s_openapi::api::rbac::v1::Role,
    ) -> Result<(), K8sPortError>;
    async fn delete_role(&self, namespace: &str, role_name: &str) -> Result<(), K8sPortError>;
    async fn list_roles(&self, namespace: &str) -> Result<Vec<String>, K8sPortError>;
    async fn create_role_binding(
        &self,
        namespace: &str,
        role_name: &str,
        user: &str,
    ) -> Result<(), K8sPortError>;
    async fn delete_role_binding(
        &self,
        namespace: &str,
        role_name: &str,
        user: &str,
    ) -> Result<(), K8sPortError>;
    async fn list_role_bindings(
        &self,
        namespace: &str,
        user: &str,
    ) -> Result<Vec<String>, K8sPortError>;
}

#[async_trait]
pub trait S3Port: Clone + Send + Sync + 'static {
    async fn get_bucket_location(&self, bucket_name: &str) -> Result<String, anyhow::Error>;
}

#[async_trait]
pub trait StsPort: Send + Sync {
    async fn assume_role(
        &self,
        role_arn: &str,
        external_id: &str,
    ) -> Result<aws_sdk_sts::types::Credentials, anyhow::Error>;
}
