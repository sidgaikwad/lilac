use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{domain::cluster::models::ClusterId, identifier};

identifier!(QueueId);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Queue {
    pub id: QueueId,
    pub name: String,
    pub priority: i32,
    pub cluster_targets: Vec<ClusterId>,
}

/// DTO for creating a new queue.
pub struct CreateQueueRequest {
    pub name: String,
    pub priority: i32,
    pub cluster_targets: Vec<ClusterId>,
}

/// DTO for updating an existing queue.
pub struct UpdateQueueRequest {
    pub id: QueueId,
    pub name: String,
    pub priority: i32,
    pub cluster_targets: Vec<ClusterId>,
}
