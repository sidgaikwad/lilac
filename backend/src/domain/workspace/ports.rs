use async_trait::async_trait;
use thiserror::Error;

use super::models::{CreateWorkspaceRequest, Workspace, WorkspaceId};
use crate::domain::{
    cluster::models::Cluster, credentials::models::Credential, project::models::ProjectId,
    user::models::UserId,
};

#[derive(Error, Debug)]
pub enum WorkspaceRepositoryError {
    #[error("not found")]
    NotFound,
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

#[async_trait]
pub trait WorkspaceRepository: Send + Sync {
    async fn create(
        &self,
        req: &CreateWorkspaceRequest,
        owner_id: UserId,
    ) -> Result<Workspace, WorkspaceRepositoryError>;
    async fn find_by_id(&self, id: WorkspaceId) -> Result<Workspace, WorkspaceRepositoryError>;
    async fn update_connection_details(
        &self,
        id: WorkspaceId,
        status: super::models::WorkspaceStatus,
        url: &str,
    ) -> Result<(), WorkspaceRepositoryError>;
    async fn list_by_project_id(
        &self,
        project_id: ProjectId,
    ) -> Result<Vec<Workspace>, WorkspaceRepositoryError>;
}

#[derive(Error, Debug)]
pub enum ProvisionerError {
    #[error("failed to provision workspace: {0}")]
    ProvisioningFailed(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[async_trait]
pub trait WorkspaceProvisioner: Send + Sync {
    async fn provision_workspace(
        &self,
        cluster: &Cluster,
        credential: &Credential,
        workspace: &mut Workspace,
    ) -> Result<(), ProvisionerError>;
}
