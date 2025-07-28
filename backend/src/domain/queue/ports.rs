use async_trait::async_trait;
use super::models::Queue;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait QueueRepository: Send + Sync {
    async fn create(&self, queue: &Queue) -> Result<(), anyhow::Error>;
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<Queue>, anyhow::Error>;
    async fn get_all_queues_sorted(&self) -> Result<Vec<Queue>, anyhow::Error>;
    async fn update(&self, queue: &Queue) -> Result<(), anyhow::Error>;
    async fn delete(&self, id: uuid::Uuid) -> Result<(), anyhow::Error>;
}