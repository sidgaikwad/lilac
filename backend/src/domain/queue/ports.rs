use crate::domain::queue::models::QueueId;

use super::models::Queue;
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait QueueRepository: Send + Sync {
    async fn create(&self, queue: &Queue) -> Result<(), anyhow::Error>;
    async fn find_by_id(&self, id: &QueueId) -> Result<Option<Queue>, anyhow::Error>;
    async fn get_all_queues_sorted(&self) -> Result<Vec<Queue>, anyhow::Error>;
    async fn update(&self, queue: &Queue) -> Result<(), anyhow::Error>;
    async fn delete(&self, id: &QueueId) -> Result<(), anyhow::Error>;
}
