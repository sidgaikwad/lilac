use std::sync::Arc;

use async_trait::async_trait;
use thiserror::Error;
use kube::{Api, Client};
use k8s_openapi::api::core::v1::Service;
use tokio::time::{sleep, Duration};

use super::{
    models::{CreateWorkspaceRequest, Workspace},
    ports::{Provisioner, ProvisionerError, WorkspaceRepository, WorkspaceRepositoryError},
};
use crate::domain::project::models::ProjectId;
use crate::domain::user::models::UserId;

#[derive(Error, Debug)]
pub enum WorkspaceServiceError {
    #[error(transparent)]
    Repository(#[from] WorkspaceRepositoryError),
    #[error(transparent)]
    Provisioner(#[from] ProvisionerError),
    #[error("an unexpected error occurred")]
    Unexpected,
}

use super::ports::WorkspaceService;


pub struct WorkspaceServiceImpl {
    repository: Arc<dyn WorkspaceRepository>,
    provisioner: Arc<dyn Provisioner>,
    kube_client: Client,
}

impl WorkspaceServiceImpl {
    pub fn new(
        repository: Arc<dyn WorkspaceRepository>,
        provisioner: Arc<dyn Provisioner>,
        kube_client: Client,
    ) -> Self {
        Self {
            repository,
            provisioner,
            kube_client,
        }
    }
}

#[async_trait]
impl WorkspaceService for WorkspaceServiceImpl {
    async fn create_workspace(
        &self,
        req: CreateWorkspaceRequest,
        owner_id: UserId,
    ) -> Result<Workspace, WorkspaceServiceError> {
        // 1. Create the initial workspace record in the database
        let workspace = self.repository.create(&req, owner_id).await?;

        // 2. Trigger the provisioner in a background task
        let provisioner = self.provisioner.clone();
        let repository = self.repository.clone();
        let workspace_clone = workspace.clone();
        let kube_client = self.kube_client.clone();

        tokio::spawn(async move {
            let result = provisioner.provision(
                workspace_clone.id,
                workspace_clone.project_id,
                &workspace_clone.image,
                workspace_clone.cpu_millicores,
                workspace_clone.memory_mb,
                &workspace_clone.ide,
                "", // public_key is no longer used
            ).await;
            
            match result {
                Ok(token) => {
                    let services: Api<Service> = Api::namespaced(kube_client, "lilac-dev");
                    let service_name = format!("workspace-{}-svc", workspace_clone.id.0);
                    
                    let is_local = std::env::var("APP_ENV").unwrap_or_else(|_| "local".to_string()) == "local";

                    for _ in 0..30 { // Poll for 30 seconds
                        if let Ok(service) = services.get(&service_name).await {
                            if is_local {
                                if let Some(spec) = service.spec {
                                    if let Some(ports) = spec.ports {
                                        if let Some(port) = ports.get(0) {
                                            if let Some(node_port) = port.node_port {
                                                let url = format!("http://localhost:{}", node_port);
                                                if let Err(e) = repository.update_connection_details(workspace_clone.id, super::models::WorkspaceStatus::Running, &url, &token).await {
                                                    eprintln!("Failed to update workspace status: {:?}", e);
                                                }
                                                return;
                                            }
                                        }
                                    }
                                }
                            } else {
                                if let Some(status) = service.status {
                                    if let Some(ingress) = status.load_balancer {
                                        if let Some(ingress_point) = ingress.ingress.as_ref().and_then(|i| i.get(0)) {
                                            let url = ingress_point.hostname.clone().or(ingress_point.ip.clone()).unwrap_or_default();
                                            if !url.is_empty() {
                                                if let Err(e) = repository.update_connection_details(workspace_clone.id, super::models::WorkspaceStatus::Running, &url, &token).await {
                                                    eprintln!("Failed to update workspace status: {:?}", e);
                                                }
                                                return;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        sleep(Duration::from_secs(1)).await;
                    }
                    eprintln!("Failed to get external IP for workspace {}", workspace_clone.id.0);
                    if let Err(e) = repository.update_connection_details(workspace_clone.id, super::models::WorkspaceStatus::Failed, "", &token).await {
                        eprintln!("Failed to update workspace status to Failed: {:?}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to provision workspace: {:?}", e);
                    if let Err(e) = repository.update_connection_details(workspace_clone.id, super::models::WorkspaceStatus::Failed, "", "").await {
                        eprintln!("Failed to update workspace status: {:?}", e);
                    }
                }
            }
        });

        // 3. Return the initial workspace object with "Pending" status
        Ok(workspace)
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
