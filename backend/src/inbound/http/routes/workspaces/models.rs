use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::workspace::models::{Ide, Workspace, WorkspaceStatus};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateWorkspacePayload {
    pub name: String,
    pub cluster_id: Uuid,
    pub ide: Ide,
    pub image: String,
    pub cpu_millicores: i32,
    pub memory_mb: i32,
    #[serde(default)]
    pub gpu: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkspaceResponse {
    pub id: Uuid,
    pub name: String,
    pub project_id: Uuid,
    pub status: WorkspaceStatus,
    pub ide: Ide,
    pub cpu_millicores: i32,
    pub memory_mb: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Workspace> for WorkspaceResponse {
    fn from(workspace: Workspace) -> Self {
        Self {
            id: workspace.id.inner(),
            name: workspace.name,
            project_id: *workspace.project_id.inner(),
            status: workspace.status,
            ide: workspace.ide,
            cpu_millicores: workspace.cpu_millicores,
            memory_mb: workspace.memory_mb,
            created_at: workspace.created_at,
            updated_at: workspace.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ConnectionDetailsResponse {
    pub url: Option<String>,
    pub token: Option<String>,
}