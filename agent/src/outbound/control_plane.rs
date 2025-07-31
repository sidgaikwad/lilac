use async_trait::async_trait;
use reqwest::Client;
use uuid::Uuid;

use crate::config::Config;
use crate::domain::agent::{
    models::{HeartbeatRequest, HeartbeatResponse, JobDetails, JobStatus, NodeResources, NodeStatus},
    ports::ControlPlaneApi,
};

#[derive(Clone)]
pub struct ControlPlaneClient {
    client: Client,
    config: Config,
}

impl ControlPlaneClient {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }
}

#[async_trait]
impl ControlPlaneApi for ControlPlaneClient {
    async fn send_heartbeat(&self, req: HeartbeatRequest) -> anyhow::Result<HeartbeatResponse> {
        let api_key = self
            .config
            .api_key
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("API key is not configured"))?;

        let url = format!("{}/nodes/heartbeat", self.config.api_endpoint);
        let response = self
            .client
            .post(&url)
            .bearer_auth(api_key)
            .json(&req)
            .send()
            .await?;

        if response.status().is_success() {
            let heartbeat_response = response.json::<HeartbeatResponse>().await?;
            Ok(heartbeat_response)
        } else {
            Err(anyhow::anyhow!(
                "Failed to send heartbeat: {}",
                response.status()
            ))
        }
    }

    async fn get_job_details(&self, job_id: Uuid) -> anyhow::Result<JobDetails> {
        let api_key = self
            .config
            .api_key
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("API key is not configured"))?;

        let url = format!("{}/jobs/{}/details", self.config.api_endpoint, job_id);
        let response = self
            .client
            .get(&url)
            .bearer_auth(api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let job_details = response.json::<JobDetails>().await?;
            Ok(job_details)
        } else {
            Err(anyhow::anyhow!(
                "Failed to get job details: {}",
                response.status()
            ))
        }
    }
}