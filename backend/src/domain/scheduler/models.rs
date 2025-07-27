use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterSnapshot {
    pub cluster_id: Uuid,
    pub nodes: Vec<NodeSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSnapshot {
    pub name: String,
    pub available_cpu_millicores: i32,
    pub available_memory_mb: i32,
    /// Detailed information about the GPUs available on this node.
    pub gpus: Vec<GpuInfo>,
}

/// Describes a single physical GPU on a node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub model: String, // e.g., "NVIDIA A100-SXM4-40GB"
    pub memory_gb: i32, // e.g., 40
}