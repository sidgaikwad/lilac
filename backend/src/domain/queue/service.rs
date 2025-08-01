use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::queue::models::{CreateQueueRequest, QueueId, UpdateQueueRequest};

use super::{
    models::Queue,
    ports::{QueueRepository, QueueRepositoryError},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum QueueServiceError {
    #[error(transparent)]
    Repository(#[from] QueueRepositoryError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[async_trait]
pub trait QueueService: Send + Sync {
    async fn create_queue(&self, request: CreateQueueRequest) -> Result<Queue, QueueServiceError>;
    async fn get_queue_by_id(&self, queue_id: &QueueId)
        -> Result<Option<Queue>, QueueServiceError>;
    async fn list_all_queues(&self) -> Result<Vec<Queue>, QueueServiceError>;
    async fn update_queue(&self, request: UpdateQueueRequest) -> Result<Queue, QueueServiceError>;
    async fn delete_queue(&self, queue_id: &QueueId) -> Result<(), QueueServiceError>;
}

pub struct QueueServiceImpl<Q: QueueRepository> {
    queue_repo: Arc<Q>,
}

impl<Q: QueueRepository> QueueServiceImpl<Q> {
    pub fn new(queue_repo: Arc<Q>) -> Self {
        Self { queue_repo }
    }
}

#[async_trait]
impl<Q: QueueRepository> QueueService for QueueServiceImpl<Q> {
    async fn create_queue(&self, request: CreateQueueRequest) -> Result<Queue, QueueServiceError> {
        let queue = Queue {
            id: QueueId::generate(),
            name: request.name,
            priority: request.priority,
            cluster_targets: request.cluster_targets,
        };

        self.queue_repo.create(&queue).await?;

        Ok(queue)
    }

    async fn get_queue_by_id(
        &self,
        queue_id: &QueueId,
    ) -> Result<Option<Queue>, QueueServiceError> {
        Ok(self.queue_repo.find_by_id(queue_id).await?)
    }

    async fn list_all_queues(&self) -> Result<Vec<Queue>, QueueServiceError> {
        Ok(self.queue_repo.get_all_queues_sorted().await?)
    }

    async fn update_queue(
        &self,
        updated_queue: UpdateQueueRequest,
    ) -> Result<Queue, QueueServiceError> {
        let queue = Queue {
            id: updated_queue.id,
            name: updated_queue.name,
            priority: updated_queue.priority,
            cluster_targets: updated_queue.cluster_targets,
        };

        self.queue_repo.update(&queue).await?;

        Ok(queue)
    }

    async fn delete_queue(&self, queue_id: &QueueId) -> Result<(), QueueServiceError> {
        Ok(self.queue_repo.delete(queue_id).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{cluster::models::ClusterId, queue::ports::MockQueueRepository};
    use mockall::predicate::eq;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_create_queue() {
        let mut mock_repo = MockQueueRepository::new();
        let new_queue_dto = CreateQueueRequest {
            name: "test_queue".to_string(),
            priority: 10,
            cluster_targets: vec![ClusterId::generate()],
        };

        let expected_name = new_queue_dto.name.clone();
        let expected_priority = new_queue_dto.priority;
        let expected_clusters = new_queue_dto.cluster_targets.clone();
        mock_repo
            .expect_create()
            .withf(move |q: &Queue| {
                q.name == expected_name
                    && q.priority == expected_priority
                    && q.cluster_targets == expected_clusters
            })
            .times(1)
            .returning(|_| Ok(()));

        let service = QueueServiceImpl::new(Arc::new(mock_repo));
        let result = service.create_queue(new_queue_dto).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_queue_by_id_found() {
        let mut mock_repo = MockQueueRepository::new();
        let queue_id = QueueId::generate();
        let queue = Queue {
            id: queue_id,
            name: "test".to_string(),
            priority: 1,
            cluster_targets: vec![],
        };

        mock_repo
            .expect_find_by_id()
            .with(eq(queue_id))
            .times(1)
            .returning(move |_| Ok(Some(queue.clone())));

        let service = QueueServiceImpl::new(Arc::new(mock_repo));
        let result = service.get_queue_by_id(&queue_id).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_get_queue_by_id_not_found() {
        let mut mock_repo = MockQueueRepository::new();
        let queue_id = QueueId::generate();

        mock_repo
            .expect_find_by_id()
            .with(eq(queue_id))
            .times(1)
            .returning(|_| Ok(None));

        let service = QueueServiceImpl::new(Arc::new(mock_repo));
        let result = service.get_queue_by_id(&queue_id).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_list_all_queues() {
        let mut mock_repo = MockQueueRepository::new();
        let queues = vec![Queue {
            id: QueueId::generate(),
            name: "test".to_string(),
            priority: 1,
            cluster_targets: vec![],
        }];

        mock_repo
            .expect_get_all_queues_sorted()
            .times(1)
            .returning(move || Ok(queues.clone()));

        let service = QueueServiceImpl::new(Arc::new(mock_repo));
        let result = service.list_all_queues().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_update_queue() {
        let mut mock_repo = MockQueueRepository::new();
        let updated_queue_dto = UpdateQueueRequest {
            id: QueueId::generate(),
            name: "updated_queue".to_string(),
            priority: 20,
            cluster_targets: vec![],
        };

        let expected_queue = Queue {
            id: updated_queue_dto.id,
            name: updated_queue_dto.name.clone(),
            priority: updated_queue_dto.priority,
            cluster_targets: updated_queue_dto.cluster_targets.clone(),
        };

        mock_repo
            .expect_update()
            .with(eq(expected_queue))
            .times(1)
            .returning(|_| Ok(()));

        let service = QueueServiceImpl::new(Arc::new(mock_repo));
        let result = service.update_queue(updated_queue_dto).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_queue() {
        let mut mock_repo = MockQueueRepository::new();
        let queue_id = QueueId::generate();

        mock_repo
            .expect_delete()
            .with(eq(queue_id))
            .times(1)
            .returning(|_| Ok(()));

        let service = QueueServiceImpl::new(Arc::new(mock_repo));
        let result = service.delete_queue(&queue_id).await;

        assert!(result.is_ok());
    }
}
