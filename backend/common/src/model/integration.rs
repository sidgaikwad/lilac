use getset::Getters;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct AWSIntegration {
    role_arn: String,
    external_id: String,
}

impl From<Json<AWSIntegration>> for AWSIntegration {
    fn from(value: Json<AWSIntegration>) -> Self {
        value.0
    }
}

impl AWSIntegration {
    pub fn new(role_arn: String, external_id: String) -> Self {
        Self {
            role_arn,
            external_id,
        }
    }

    pub fn create(role_arn: String) -> Self {
        Self {
            role_arn,
            external_id: format!("Lilac-{}", Uuid::new_v4().as_simple()),
        }
    }
}
