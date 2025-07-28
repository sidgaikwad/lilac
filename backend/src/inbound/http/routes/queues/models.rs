use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::queue::models::Queue;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQueueRequest {
    pub name: String,
    pub priority: i32,
    pub cluster_targets: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQueueRequest {
    pub name: String,
    pub priority: i32,
    pub cluster_targets: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueueResponse {
    pub id: Uuid,
    pub name: String,
    pub priority: i32,
    pub cluster_targets: Vec<Uuid>,
}

impl From<Queue> for QueueResponse {
    fn from(queue: Queue) -> Self {
        Self {
            id: queue.id,
            name: queue.name,
            priority: queue.priority,
            cluster_targets: queue.cluster_targets,
        }
    }
}