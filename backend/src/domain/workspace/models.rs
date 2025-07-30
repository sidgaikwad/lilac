use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{cluster::models::ClusterId, project::models::ProjectId, user::models::UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkspaceId(pub Uuid);

impl WorkspaceId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn inner(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for WorkspaceId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl Display for WorkspaceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Ide {
    Vscode,
    JupyterLab,
    RStudio,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkspaceStatus {
    Pending,
    Running,
    Stopping,
    Stopped,
    Failed,
    Terminated,
}

#[derive(Debug, Clone, Serialize)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub name: String,
    pub project_id: ProjectId,
    pub owner_id: UserId,
    pub cluster_id: ClusterId,
    pub ide: Ide,
    pub image: String,
    pub cpu_millicores: i32,
    pub memory_mb: i32,
    pub gpu: bool,
    pub status: WorkspaceStatus,
    pub url: Option<String>,
    pub token: Option<String>,
    pub public_key: Option<String>,
    pub private_key: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
    pub project_id: ProjectId,
    pub cluster_id: ClusterId,
    pub ide: Ide,
    pub image: String,
    pub cpu_millicores: i32,
    pub memory_mb: i32,
    pub gpu: bool,
}
