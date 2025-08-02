use std::sync::Arc;

use async_trait::async_trait;
use thiserror::Error;

use super::{
    models::{CreateWorkspaceRequest, Workspace},
    ports::{
        ProvisionerError, WorkspaceProvisioner, WorkspaceRepository, WorkspaceRepositoryError,
    },
};
use crate::domain::{
    cluster::ports::{ClusterRepository, ClusterRepositoryError},
    credentials::ports::{CredentialRepository, CredentialRepositoryError},
    project::models::ProjectId,
    user::models::UserId,
    workspace::models::WorkspaceId,
};

#[derive(Error, Debug)]
pub enum WorkspaceServiceError {
    #[error(transparent)]
    Repository(#[from] WorkspaceRepositoryError),
    #[error(transparent)]
    Provisioner(#[from] ProvisionerError),
    #[error(transparent)]
    ClusterRepository(#[from] ClusterRepositoryError),
    #[error(transparent)]
    CredentialRepository(#[from] CredentialRepositoryError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

use crate::config::LilacConfig;

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

pub struct WorkspaceServiceImpl {
    repository: Arc<dyn WorkspaceRepository>,
    provisioner: Arc<dyn WorkspaceProvisioner>,
    cluster_repository: Arc<dyn ClusterRepository>,
    credential_repository: Arc<dyn CredentialRepository>,
    config: Arc<LilacConfig>,
}

impl WorkspaceServiceImpl {
    pub fn new(
        repository: Arc<dyn WorkspaceRepository>,
        provisioner: Arc<dyn WorkspaceProvisioner>,
        cluster_repository: Arc<dyn ClusterRepository>,
        credential_repository: Arc<dyn CredentialRepository>,
        config: Arc<LilacConfig>,
    ) -> Self {
        Self {
            repository,
            provisioner,
            cluster_repository,
            credential_repository,
            config,
        }
    }
}

#[async_trait]
impl WorkspaceService for WorkspaceServiceImpl {
    async fn create_workspace(
        &self,
        _req: CreateWorkspaceRequest,
        _owner_id: UserId,
    ) -> Result<Workspace, WorkspaceServiceError> {
        todo!();
        // // 1. Create the initial workspace record in the database
        // let workspace = self.repository.create(&req, owner_id).await?;

        // // 2. Get the cluster and create a kube client for it
        // let cluster = self
        //     .cluster_repository
        //     .get_cluster_by_id(&req.cluster_id)
        //     .await?;
        // let credential = self
        //     .credential_repository
        //     .get_credential_by_id(&cluster.credential_id)
        //     .await?;

        // // 3. Trigger the provisioner in a background task
        // let provisioner = self.provisioner.clone();
        // let mut workspace_clone = workspace.clone();
        // let workspace_repository = self.repository.clone();
        // tokio::spawn(async move {
        //     let result = provisioner
        //         .provision_workspace(&cluster, &credential, &mut workspace_clone)
        //         .await;
        //     match result {
        //         Ok(()) => {
        //             let url = workspace_clone.url.as_ref().expect("url to be set");
        //             let res = workspace_repository
        //                 .update_connection_details(
        //                     workspace_clone.id,
        //                     WorkspaceStatus::Running,
        //                     url,
        //                 )
        //                 .await;
        //             if let Err(err) = res {
        //                 tracing::error!(error = ?err, workspace_id = %workspace_clone.id, url = %url, "failed to update workspace connection details");
        //             }
        //         }
        //         Err(err) => {
        //             tracing::error!(error = ?err, workspace_id = %workspace_clone.id, "failed to allocate workspace");
        //         }
        //     }
        // });

        // // 3. Return the initial workspace object with "Pending" status
        // Ok(workspace)
    }

    async fn list_workspaces(
        &self,
        project_id: ProjectId,
        _owner_id: UserId,
    ) -> Result<Vec<super::models::Workspace>, WorkspaceServiceError> {
        // TODO: Check if the owner_id has access to the project
        let workspaces = self.repository.list_by_project_id(project_id).await?;
        Ok(workspaces)
    }

    async fn find_by_id(
        &self,
        workspace_id: super::models::WorkspaceId,
        _owner_id: UserId,
    ) -> Result<Workspace, WorkspaceServiceError> {
        // TODO: Check if the owner_id matches the workspace owner
        let workspace = self.repository.find_by_id(workspace_id).await?;
        Ok(workspace)
    }
}
