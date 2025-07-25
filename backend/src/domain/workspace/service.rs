use std::sync::Arc;

use async_trait::async_trait;
use thiserror::Error;
use kube::Api;
use k8s_openapi::api::core::v1::Service;
use tokio::time::{sleep, Duration};

use super::{
    models::{CreateWorkspaceRequest, Workspace},
    ports::{Provisioner, ProvisionerError, WorkspaceRepository, WorkspaceRepositoryError},
};
use crate::{
    domain::{
        cluster::ports::{ClusterRepository, ClusterRepositoryError},
        project::models::ProjectId,
        user::models::UserId,
    },
    outbound::k8s::factory::{KubeClientFactory, KubeClientFactoryError},
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
    KubeClientFactory(#[from] KubeClientFactoryError),
    #[error("an unexpected error occurred")]
    Unexpected,
}

use crate::config::LilacConfig;

use super::ports::WorkspaceService;

pub struct WorkspaceServiceImpl {
    repository: Arc<dyn WorkspaceRepository>,
    provisioner: Arc<dyn Provisioner>,
    cluster_repository: Arc<dyn ClusterRepository>,
    kube_client_factory: Arc<dyn KubeClientFactory>,
    config: Arc<LilacConfig>,
}

impl WorkspaceServiceImpl {
    pub fn new(
        repository: Arc<dyn WorkspaceRepository>,
        provisioner: Arc<dyn Provisioner>,
        cluster_repository: Arc<dyn ClusterRepository>,
        kube_client_factory: Arc<dyn KubeClientFactory>,
        config: Arc<LilacConfig>,
    ) -> Self {
        Self {
            repository,
            provisioner,
            cluster_repository,
            kube_client_factory,
            config,
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

        // 2. Get the cluster and create a kube client for it
        let cluster = self
            .cluster_repository
            .get_cluster_by_id(&req.cluster_id)
            .await?;
        let kube_client = self.kube_client_factory.create_client(&cluster).await?;

        // 3. Trigger the provisioner in a background task
        let provisioner = self.provisioner.clone();
        let repository = self.repository.clone();
        let workspace_clone = workspace.clone();
        let config = self.config.clone();

        tokio::spawn(async move {
            let result = provisioner
                .provision(
                    &kube_client,
                    workspace_clone.id,
                    workspace_clone.project_id,
                    "themrtoadog/jupyter-lilac:latest",
                    workspace_clone.cpu_millicores,
                    workspace_clone.memory_mb,
                    &workspace_clone.ide,
                    "", // public_key is no longer used
                   workspace_clone.gpu,
                )
                .await;

            match result {
                Ok(token) => {
                    let services: Api<Service> =
                        Api::namespaced(kube_client, &config.kubernetes_namespace);
                    let service_name = format!("workspace-{}-svc", workspace_clone.id.0);

                    for _ in 0..300 {
                        // Poll for 5 minutes
                        if let Ok(service) = services.get(&service_name).await {
                            match cluster.cluster_config {
                                crate::domain::cluster::models::ClusterConfig::Local => {
                                    if let Some(spec) = service.spec {
                                        if let Some(ports) = spec.ports {
                                            if let Some(port) = ports.get(0) {
                                                if let Some(node_port) = port.node_port {
                                                    let url =
                                                        format!("http://localhost:{}", node_port);
                                                    if let Err(e) = repository
                                                        .update_connection_details(
                                                            workspace_clone.id,
                                                            super::models::WorkspaceStatus::Running,
                                                            &url,
                                                            &token,
                                                        )
                                                        .await
                                                    {
                                                        eprintln!(
                                                            "Failed to update workspace status: {:?}",
                                                            e
                                                        );
                                                    }
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                                crate::domain::cluster::models::ClusterConfig::AwsEks { .. } => {
                                    if let Some(status) = service.status {
                                        if let Some(ingress) = status.load_balancer {
                                            if let Some(ingress_point) =
                                                ingress.ingress.as_ref().and_then(|i| i.get(0))
                                            {
                                                let url = ingress_point
                                                    .hostname
                                                    .clone()
                                                    .or(ingress_point.ip.clone())
                                                    .unwrap_or_default();
                                                if !url.is_empty() {
                                                    if let Err(e) = repository
                                                        .update_connection_details(
                                                            workspace_clone.id,
                                                            super::models::WorkspaceStatus::Running,
                                                            &url,
                                                            &token,
                                                        )
                                                        .await
                                                    {
                                                        eprintln!(
                                                            "Failed to update workspace status: {:?}",
                                                            e
                                                        );
                                                    }
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                },
                                crate::domain::cluster::models::ClusterConfig::GcpGke { .. } => {
                                    if let Some(status) = service.status {
                                        if let Some(ingress) = status.load_balancer {
                                            if let Some(ingress_point) =
                                                ingress.ingress.as_ref().and_then(|i| i.get(0))
                                            {
                                                let url = ingress_point
                                                    .hostname
                                                    .clone()
                                                    .or(ingress_point.ip.clone())
                                                    .unwrap_or_default();
                                                if !url.is_empty() {
                                                    if let Err(e) = repository
                                                        .update_connection_details(
                                                            workspace_clone.id,
                                                            super::models::WorkspaceStatus::Running,
                                                            &url,
                                                            &token,
                                                        )
                                                        .await
                                                    {
                                                        eprintln!(
                                                            "Failed to update workspace status: {:?}",
                                                            e
                                                        );
                                                    }
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        sleep(Duration::from_secs(1)).await;
                    }
                    eprintln!(
                        "Failed to get external IP for workspace {}",
                        workspace_clone.id.0
                    );
                    if let Err(e) = repository
                        .update_connection_details(
                            workspace_clone.id,
                            super::models::WorkspaceStatus::Failed,
                            "",
                            &token,
                        )
                        .await
                    {
                        eprintln!("Failed to update workspace status to Failed: {:?}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to provision workspace: {:?}", e);
                    if let Err(e) = repository
                        .update_connection_details(
                            workspace_clone.id,
                            super::models::WorkspaceStatus::Failed,
                            "",
                            "",
                        )
                        .await
                    {
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
