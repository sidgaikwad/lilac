use serde::{Deserialize, Serialize};

use crate::domain::{
    cluster::models::ClusterId,
    queue::models::{Queue, QueueId},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HttpCreateQueueRequest {
    pub name: String,
    pub priority: i32,
    pub cluster_targets: Vec<ClusterId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HttpUpdateQueueRequest {
    pub name: String,
    pub priority: i32,
    pub cluster_targets: Vec<ClusterId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HttpQueueResponse {
    pub id: QueueId,
    pub name: String,
    pub priority: i32,
    pub cluster_targets: Vec<ClusterId>,
}

impl From<Queue> for HttpQueueResponse {
    fn from(queue: Queue) -> Self {
        Self {
            id: queue.id,
            name: queue.name,
            priority: queue.priority,
            cluster_targets: queue.cluster_targets,
        }
    }
}
