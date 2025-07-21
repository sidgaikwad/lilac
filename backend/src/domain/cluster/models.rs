use std::fmt::Display;

use crate::domain::credentials::models::CredentialId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ClusterId(pub Uuid);

impl ClusterId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn inner(&self) -> &Uuid {
        &self.0
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Display for ClusterId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for ClusterId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<ClusterId> for Uuid {
    fn from(id: ClusterId) -> Self {
        id.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClusterConfig {
    Local,
    AwsEks {
        cluster_name: String,
        region: String,
    },
    GcpGke {
        project_id: String,
        region: String,
        cluster_name: String,
    },
}

#[derive(Clone, Debug)]
pub struct Cluster {
    pub id: ClusterId,
    pub name: String,
    pub description: Option<String>,
    pub cluster_config: ClusterConfig,
    pub credential_id: CredentialId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct CreateClusterRequest {
    pub name: String,
    pub description: Option<String>,
    pub cluster_config: ClusterConfig,
    pub credential_id: CredentialId,
}
