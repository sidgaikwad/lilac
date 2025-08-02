use crate::{config::UserConfig, errors::UserApiError};
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct GpuRequirement {
    pub count: i32,
    pub model: Option<String>,
    pub memory_gb: Option<i32>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ResourceRequirements {
    pub cpu_millicores: i32,
    pub memory_mb: i32,
    pub gpus: Option<GpuRequirement>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SubmitJobRequest {
    pub name: String,
    pub definition: String, // Docker image
    pub queue_id: String,
    pub resource_requirements: ResourceRequirements,
}

#[derive(Deserialize, Debug)]
pub struct SubmitJobResponse {
    pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Queue {
    pub id: String,
    pub name: String,
}

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    config: UserConfig,
}

impl ApiClient {
    pub fn new(config: UserConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    fn add_auth(&self, req_builder: RequestBuilder) -> RequestBuilder {
        if let Some(token) = &self.config.api_key {
            req_builder.bearer_auth(token)
        } else {
            req_builder
        }
    }

    pub async fn submit_job(
        &self,
        request: SubmitJobRequest,
    ) -> Result<SubmitJobResponse, UserApiError> {
        let url = format!("{}/training_jobs", self.config.api_endpoint);

        let req_builder = self.client.post(&url).json(&request);
        let req_builder = self.add_auth(req_builder);

        let response = req_builder.send().await?;

        match response.status() {
            StatusCode::OK => {
                let job_response = response.json::<SubmitJobResponse>().await?;
                Ok(job_response)
            }
            StatusCode::UNAUTHORIZED => Err(UserApiError::Unauthorized),
            StatusCode::NOT_FOUND => Err(UserApiError::NotFound),
            StatusCode::INTERNAL_SERVER_ERROR => Err(UserApiError::InternalServerError),
            _ => Err(UserApiError::Unknown(anyhow::anyhow!(
                "Failed to submit job: {}",
                response.status()
            ))),
        }
    }

    pub async fn get_queues(&self) -> Result<Vec<Queue>, UserApiError> {
        let url = format!("{}/queues", self.config.api_endpoint);

        let req_builder = self.client.get(&url);
        let req_builder = self.add_auth(req_builder);

        let response = req_builder.send().await?;

        match response.status() {
            StatusCode::OK => {
                let queues = response.json::<Vec<Queue>>().await?;
                Ok(queues)
            }
            StatusCode::UNAUTHORIZED => Err(UserApiError::Unauthorized),
            StatusCode::NOT_FOUND => Err(UserApiError::NotFound),
            StatusCode::INTERNAL_SERVER_ERROR => Err(UserApiError::InternalServerError),
            _ => Err(UserApiError::Unknown(anyhow::anyhow!(
                "Failed to get queues: {}",
                response.status()
            ))),
        }
    }
}