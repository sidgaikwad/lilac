use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct ClusterId(pub Uuid);
impl From<Uuid> for ClusterId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

#[derive(Clone, Debug)]
pub enum ClusterConfig {
    K8s()
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Cluster {
    pub id: ClusterId,
    pub name: String,
    pub description: Option<String>,
    pub cluster_config: ClusterConfig,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Validate)]
pub struct CreateClusterRequest {
    #[validate(length(min = 1))]
    pub name: String,
}
