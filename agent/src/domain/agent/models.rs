use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents the static hardware resources of a compute node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeResources {
    pub cpu: CpuConfiguration,
    pub gpus: Vec<GpuConfiguration>,
    pub memory_mb: u64,
}

/// Detailed information about the CPU.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuConfiguration {
    pub manufacturer: String,
    pub architecture: String,
    pub millicores: u32,
}

/// Detailed information about a single GPU.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuConfiguration {
    pub manufacturer: String,
    pub model_name: String,
    pub memory_mb: u32,
}

/// The status of a node, reported during a heartbeat.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NodeStatus {
    Available,
    Busy,
}

/// The response from a heartbeat call, which may include a job to run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatResponse {
    pub assigned_job_id: Option<Uuid>,
}

/// The full details of a job, fetched by the agent when assigned.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobDetails {
    pub id: Uuid,
    /// The Docker image URI for the job.
    pub definition: String,
}

/// The status of a job, reported by the agent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    Running,
    Succeeded,
    Failed,
}