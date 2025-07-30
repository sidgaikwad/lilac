use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::cluster::models::{
    Cluster, ClusterId, ClusterNode, Cpu, CreateClusterRequest, Gpu, JobInfo, NodeId, NodeStatus,
};

/// The body of a [Cluster] creation request.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateClusterHttpRequest {
    cluster_name: String,
    cluster_description: Option<String>,
}

impl From<CreateClusterHttpRequest> for CreateClusterRequest {
    fn from(value: CreateClusterHttpRequest) -> Self {
        CreateClusterRequest {
            name: value.cluster_name,
            description: value.cluster_description,
        }
    }
}

/// The body of a [Cluster] creation response.
#[derive(Debug, Clone, Serialize)]
pub struct CreateClusterHttpResponse {
    pub cluster_id: ClusterId,
}

/// The body of a [Cluster] get response.
#[derive(Debug, Clone, Serialize)]
pub struct GetClusterHttpResponse {
    pub cluster_id: ClusterId,
    pub cluster_name: String,
    pub cluster_description: Option<String>,
}

impl From<Cluster> for GetClusterHttpResponse {
    fn from(value: Cluster) -> Self {
        Self {
            cluster_id: value.id,
            cluster_name: value.name,
            cluster_description: value.description,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct HttpClusterSummary {
    pub cluster_id: ClusterId,
    pub cluster_name: String,
    pub cluster_description: Option<String>,
}

impl From<Cluster> for HttpClusterSummary {
    fn from(cluster: Cluster) -> Self {
        Self {
            cluster_id: cluster.id,
            cluster_name: cluster.name,
            cluster_description: cluster.description,
        }
    }
}

/// The body of a [Cluster] list response.
#[derive(Clone, Debug, Serialize)]
pub struct ListClustersHttpResponse {
    pub clusters: Vec<HttpClusterSummary>,
}

impl From<Vec<Cluster>> for ListClustersHttpResponse {
    fn from(value: Vec<Cluster>) -> Self {
        Self {
            clusters: value.into_iter().map(HttpClusterSummary::from).collect(),
        }
    }
}

/// The body of a [ClusterNode] heartbeat request.
#[derive(Debug, Clone, Deserialize)]
pub struct HttpClusterNodeHeartbeat {
    pub status: NodeStatus,
    pub memory_info: i32,
    pub cpu_info: Cpu,
    pub gpu_info: Option<Gpu>,
    pub job_info: Option<JobInfo>,
}

/// The body of a [Cluster] list response.
#[derive(Clone, Debug, Serialize)]
pub struct HttpClusterNodeDetails {}

/// The body of a [ClusterNode] get request.
#[derive(Debug, Clone, Serialize)]
pub struct HttpClusterNode {
    pub id: NodeId,
    pub cluster_id: ClusterId,
    pub node_status: NodeStatus,
    pub last_heartbeat: DateTime<Utc>,
    pub memory_mb: i32,
    pub cpu: Cpu,
    pub gpu: Option<Gpu>,
}

impl From<ClusterNode> for HttpClusterNode {
    fn from(value: ClusterNode) -> Self {
        Self {
            id: value.id,
            cluster_id: value.cluster_id,
            node_status: value.node_status,
            last_heartbeat: value.heartbeat_timestamp,
            memory_mb: value.memory_mb,
            cpu: value.cpu,
            gpu: value.gpu,
        }
    }
}

/// The body of a [Cluster] list response.
#[derive(Clone, Debug, Serialize)]
pub struct ListClusterNodesHttpResponse {
    pub cluster_nodes: Vec<HttpClusterNode>,
}

impl From<Vec<ClusterNode>> for ListClusterNodesHttpResponse {
    fn from(value: Vec<ClusterNode>) -> Self {
        Self {
            cluster_nodes: value.into_iter().map(HttpClusterNode::from).collect(),
        }
    }
}
