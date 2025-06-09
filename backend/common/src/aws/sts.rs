use std::sync::Arc;

use aws_config::{AppName, BehaviorVersion, Region};
use aws_sdk_sts::{
    operation::assume_role::AssumeRoleError, types::Credentials, Client as STSClient,
};
use tracing::instrument;

use crate::ServiceError;

#[derive(Debug, thiserror::Error)]
pub enum STSError {
    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("{0}")]
    AssumeRoleError(#[from] AssumeRoleError),
}

impl From<STSError> for ServiceError {
    fn from(value: STSError) -> ServiceError {
        match value {
            STSError::InvalidCredentials => ServiceError::UnhandledError,
            STSError::AssumeRoleError(_) => ServiceError::UnhandledError,
        }
    }
}

#[derive(Clone, Debug)]
pub struct STSWrapper {
    client: Arc<STSClient>,
}

impl STSWrapper {
    pub fn new(client: STSClient) -> Self {
        Self {
            client: Arc::new(client),
        }
    }

    pub async fn new_from_default() -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .app_name(AppName::new("lilac").expect("valid app name"))
            .region(Region::new("us-west-2"))
            .load()
            .await;
        let client = STSClient::new(&config);
        Self {
            client: Arc::new(client),
        }
    }

    #[instrument(level = "info", skip(self), err)]
    pub async fn assume_role(
        &self,
        role_arn: &str,
        external_id: &str,
    ) -> Result<Credentials, STSError> {
        let resp = self
            .client
            .assume_role()
            .role_arn(role_arn)
            .role_session_name("lilac")
            .external_id(external_id)
            .send()
            .await
            .map_err(|e| e.into_service_error())?;
        resp.credentials.ok_or(STSError::InvalidCredentials)
    }
}
