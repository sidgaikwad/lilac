use async_trait::async_trait;
use thiserror::Error;

use super::models::{CreateWorkspaceRequest, Ide, Workspace, WorkspaceId};
use crate::domain::{
    project::models::ProjectId, user::models::UserId,
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
        token: &str,
    ) -> Result<(), WorkspaceRepositoryError>;
    async fn list_by_project_id(
        &self,
        project_id: ProjectId,
    ) -> Result<Vec<Workspace>, WorkspaceRepositoryError>;
}

#[derive(Error, Debug)]
pub enum ProvisionerError {
    #[error("failed to provision")]
    Failed,
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

use kube::Client;

#[async_trait]
pub trait Provisioner: Send + Sync {
    async fn provision(
        &self,
        client: &Client,
        workspace_id: WorkspaceId,
        project_id: ProjectId,
        image: &str,
        cpu_millicores: i32,
        memory_mb: i32,
        ide: &Ide,
        public_key: &str,
    ) -> Result<String, ProvisionerError>;
}

#[async_trait]
pub trait WorkspaceService: Send + Sync {
    async fn create_workspace(
        &self,
        req: CreateWorkspaceRequest,
        owner_id: UserId,
    ) -> Result<Workspace, super::service::WorkspaceServiceError>;
    async fn list_workspaces(
        &self,
        project_id: ProjectId,
        owner_id: UserId,
    ) -> Result<Vec<Workspace>, super::service::WorkspaceServiceError>;
    async fn find_by_id(
        &self,
        workspace_id: WorkspaceId,
        owner_id: UserId,
    ) -> Result<Workspace, super::service::WorkspaceServiceError>;
}