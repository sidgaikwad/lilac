use async_trait::async_trait;
use reqwest::Client;
use uuid::Uuid;

use crate::config::Config;
use crate::domain::agent::{
    models::{HeartbeatResponse, JobDetails, JobStatus, NodeResources, NodeStatus},
    ports::ControlPlaneApi,
};

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ControlPlaneClient {
    _client: Client,
    config: Config,
    // A simple counter to simulate job assignment after a few heartbeats.
    heartbeat_count: Arc<Mutex<u32>>,
}

impl ControlPlaneClient {
    pub fn new(config: Config) -> Self {
        Self {
            _client: Client::new(),
            config,
            heartbeat_count: Arc::new(Mutex::new(0)),
        }
    }
}

#[async_trait]
impl ControlPlaneApi for ControlPlaneClient {
    async fn register_node(&self, resources: NodeResources) -> anyhow::Result<()> {
        let _url = format!("{}/nodes", self.config.api_endpoint);
        // This is a placeholder. The actual implementation will be added once the
        // backend API is ready. We will mock this for now.
        println!("[MOCK] Registering node with resources: {:?}", resources);
        Ok(())
    }

    async fn send_heartbeat(&self, status: NodeStatus) -> anyhow::Result<HeartbeatResponse> {
        let mut count = self.heartbeat_count.lock().unwrap();
        *count += 1;

        println!("[MOCK] Sending heartbeat #{} with status: {:?}", *count, status);

        // After the 3rd heartbeat, assign a job.
        if *count == 3 {
            println!("[MOCK] Assigning mock job...");
            Ok(HeartbeatResponse {
                assigned_job_id: Some(Uuid::new_v4()),
            })
        } else {
            Ok(HeartbeatResponse {
                assigned_job_id: None,
            })
        }
    }

    async fn get_job_details(&self, job_id: Uuid) -> anyhow::Result<JobDetails> {
        let _url = format!("{}/jobs/{}", self.config.api_endpoint, job_id);
        println!("[MOCK] Getting job details for job ID: {}", job_id);
        Ok(JobDetails {
            id: job_id,
            // Use a minimal, multi-architecture image for the test.
            definition: "alpine:latest".to_string(),
        })
    }

    async fn update_job_status(&self, job_id: Uuid, status: JobStatus) -> anyhow::Result<()> {
        let _url = format!("{}/jobs/{}/status", self.config.api_endpoint, job_id);
        // This is a placeholder. The actual implementation will be added once the
        // backend API is ready. We will mock this for now.
        println!(
            "[MOCK] Updating job status for job ID: {} to {:?}",
            job_id, status
        );
        Ok(())
    }
}