use crate::{
    config::AgentConfig,
    domain::agent::{
        models::{HeartbeatRequest, HeartbeatResponse, JobDetails},
        ports::ControlPlaneApi,
    },
    errors::ControlPlaneApiError,
};
use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use uuid::Uuid;

#[derive(Clone)]
pub struct ControlPlaneClient {
    client: Client,
    config: AgentConfig,
}

impl ControlPlaneClient {
    pub fn new(config: AgentConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }
}

#[async_trait]
impl ControlPlaneApi for ControlPlaneClient {
    async fn send_heartbeat(
        &self,
        node_id: Uuid,
        req: HeartbeatRequest,
    ) -> Result<HeartbeatResponse, ControlPlaneApiError> {
        let api_key = &self.config.cluster_api_key;

        let url = format!("{}/node/{}/status", self.config.api_endpoint, node_id);
        let response = self
            .client
            .post(&url)
            .bearer_auth(api_key)
            .json(&req)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let heartbeat_response = response.json::<HeartbeatResponse>().await?;
                Ok(heartbeat_response)
            }
            StatusCode::UNAUTHORIZED => Err(ControlPlaneApiError::Unauthorized),
            StatusCode::NOT_FOUND => Err(ControlPlaneApiError::NotFound),
            StatusCode::INTERNAL_SERVER_ERROR => Err(ControlPlaneApiError::InternalServerError),
            _ => Err(ControlPlaneApiError::Unknown(anyhow::anyhow!(
                "Failed to send heartbeat: {}",
                response.status()
            ))),
        }
    }

    async fn get_job_details(
        &self,
        job_id: Uuid,
    ) -> Result<JobDetails, ControlPlaneApiError> {
        let api_key = &self.config.cluster_api_key;

        let url = format!("{}/jobs/{}/details", self.config.api_endpoint, job_id);
        let response = self
            .client
            .get(&url)
            .bearer_auth(api_key)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let job_details = response.json::<JobDetails>().await?;
                Ok(job_details)
            }
            StatusCode::UNAUTHORIZED => Err(ControlPlaneApiError::Unauthorized),
            StatusCode::NOT_FOUND => Err(ControlPlaneApiError::NotFound),
            StatusCode::INTERNAL_SERVER_ERROR => Err(ControlPlaneApiError::InternalServerError),
            _ => Err(ControlPlaneApiError::Unknown(anyhow::anyhow!(
                "Failed to get job details: {}",
                response.status()
            ))),
        }
    }
}