use async_trait::async_trait;
use aws_config::{AppName, BehaviorVersion, Region};
use aws_sdk_sts::{types::Credentials, Client as STSClient};
use tracing::instrument;

use crate::domain::integration::ports::StsPort;


#[derive(Clone, Debug)]
pub struct StsAdapter {
    client: STSClient,
}

impl StsAdapter {
    pub async fn new() -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .app_name(AppName::new("lilac").expect("valid app name"))
            .region(Region::new("us-west-2"))
            .load()
            .await;
        let client = STSClient::new(&config);
        Self { client }
    }
}

#[async_trait]
impl StsPort for StsAdapter {
    #[instrument(level = "info", skip(self), err)]
    async fn assume_role(
        &self,
        role_arn: &str,
        external_id: &str,
    ) -> Result<Credentials, anyhow::Error> {
        let resp = self
            .client
            .assume_role()
            .role_arn(role_arn)
            .role_session_name("lilac")
            .external_id(external_id)
            .send()
            .await
            .map_err(|e| e.into_service_error())?;
        resp.credentials.ok_or_else(|| anyhow::anyhow!("Invalid credentials"))
    }
}