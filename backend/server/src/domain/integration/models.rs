use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::project::models::ProjectId;

#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct IntegrationId(pub Uuid);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Integration {
    AWS(AWSIntegration),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "jsonb")]
pub struct AWSIntegration {
    pub id: IntegrationId,
    pub project_id: ProjectId,
    pub role_arn: String,
    pub external_id: String,
}

#[derive(Debug)]
pub struct CreateAWSIntegrationRequest {
    pub project_id: ProjectId,
    pub role_arn: String,
}
