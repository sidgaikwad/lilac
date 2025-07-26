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
    pub available_gpus: i32,
}