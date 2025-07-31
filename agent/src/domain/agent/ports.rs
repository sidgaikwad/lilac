use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::agent::models::{NodeResources, HeartbeatResponse, JobDetails, HeartbeatRequest};

/// Port for interacting with the Lilac control plane API. update to do proper error handling
#[async_trait]
pub trait ControlPlaneApi: Send + Sync {
    /// Sends a heartbeat to the control plane, reporting the current node status.
    /// Returns a potential job if the control plane has assigned one.
    async fn send_heartbeat(&self, req: HeartbeatRequest) -> anyhow::Result<HeartbeatResponse>;

    /// Fetches the full details for an assigned job.
    async fn get_job_details(&self, job_id: Uuid) -> anyhow::Result<JobDetails>;
}

/// Port for monitoring the local system's hardware resources.
#[async_trait]
pub trait SystemMonitor: Send + Sync {
    /// Gathers information about the system's CPU, memory, and GPUs.
    async fn get_node_resources(&self) -> anyhow::Result<NodeResources>;
}

/// Port for executing jobs, typically in a containerized environment.
#[async_trait]
pub trait JobExecutor: Send + Sync {
    /// Runs the specified job and returns the exit code.
    async fn run_job(&self, job_details: JobDetails) -> anyhow::Result<i64>;
}