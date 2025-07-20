use async_trait::async_trait;
use kube::{
    api::{Api, PostParams},
    Client,
};
use k8s_openapi::api::{
    apps::v1::Deployment,
    core::v1::{Service, Secret},
};
use serde_json::json;
use std::collections::BTreeMap;
use uuid::Uuid;


use crate::domain::{
    project::models::ProjectId,
    workspace::{
        models::{Ide, WorkspaceId},
        ports::{Provisioner, ProvisionerError},
    },
};

pub struct KubernetesProvisioner {
    client: Client,
}

impl KubernetesProvisioner {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Provisioner for KubernetesProvisioner {
    async fn provision(
        &self,
        workspace_id: WorkspaceId,
        _project_id: ProjectId,
        image: &str,
        cpu_millicores: i32,
        memory_mb: i32,
        _ide: &Ide,
        public_key: &str,
    ) -> Result<String, ProvisionerError> {
        let namespace = "lilac-dev";
        let workspace_name = format!("workspace-{}", workspace_id.0);
        let token = Uuid::new_v4().to_string();

        // 1. Create Deployment
        let deployments: Api<Deployment> = Api::namespaced(self.client.clone(), namespace);

        let deployment = serde_json::from_value(json!({
            "apiVersion": "apps/v1",
            "kind": "Deployment",
            "metadata": {
                "name": workspace_name,
                "namespace": namespace,
                "labels": {
                    "app": workspace_name
                }
            },
            "spec": {
                "replicas": 1,
                "selector": {
                    "matchLabels": {
                        "app": workspace_name
                    }
                },
                "template": {
                    "metadata": {
                        "labels": {
                            "app": workspace_name
                        }
                    },
                    "spec": {
                        "containers": [{
                            "name": "workspace",
                            "image": image,
                            "imagePullPolicy": "Never",
                            "command": [
                                "start-notebook.sh",
                                &format!("--ServerApp.token={}", token),
                                "--ServerApp.allow_origin='*'",
                            ],
                            "ports": [
                                {"containerPort": 8888}
                            ],
                            "resources": {
                                "requests": {
                                    "cpu": format!("{}m", cpu_millicores),
                                    "memory": format!("{}Mi", memory_mb)
                                },
                                "limits": {
                                    "cpu": format!("{}m", cpu_millicores),
                                    "memory": format!("{}Mi", memory_mb)
                                }
                            }
                        }]
                    }
                }
            }
        })).map_err(|e| ProvisionerError::Other(e.into()))?;

        deployments.create(&PostParams::default(), &deployment).await.map_err(|e| ProvisionerError::Other(e.into()))?;

        // 2. Create Service
        let services: Api<Service> = Api::namespaced(self.client.clone(), namespace);

        let service = serde_json::from_value(json!({
            "apiVersion": "v1",
            "kind": "Service",
            "metadata": {
                "name": format!("{}-svc", workspace_name),
                "namespace": namespace
            },
            "spec": {
                "selector": {
                    "app": workspace_name
                },
                "ports": [
                    {"name": "http", "port": 80, "targetPort": 8888}
                ],
                "type": "LoadBalancer"
            }
        })).map_err(|e| ProvisionerError::Other(e.into()))?;

        services.create(&PostParams::default(), &service).await.map_err(|e| ProvisionerError::Other(e.into()))?;

        // 3. Return the token
        Ok(token)
    }
}