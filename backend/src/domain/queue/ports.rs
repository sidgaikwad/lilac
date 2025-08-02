use crate::domain::queue::models::QueueId;

use super::models::Queue;
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum QueueRepositoryError {
    #[error("queue with {field} {value} already exists")]
    Duplicate { field: String, value: String },
    #[error("queue with id {0} not found")]
    NotFound(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait QueueRepository: Send + Sync {
    async fn create(&self, queue: &Queue) -> Result<(), QueueRepositoryError>;
    async fn get_queue_by_id(&self, id: &QueueId) -> Result<Queue, QueueRepositoryError>;
    async fn get_all_queues_sorted(&self) -> Result<Vec<Queue>, QueueRepositoryError>;
    async fn update(&self, queue: &Queue) -> Result<(), QueueRepositoryError>;
    async fn delete(&self, id: &QueueId) -> Result<(), QueueRepositoryError>;
}
