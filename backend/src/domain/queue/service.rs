use std::sync::Arc;

use uuid::Uuid;

use super::{models::Queue, ports::QueueRepository};

/// DTO for creating a new queue.
pub struct CreateQueue {
    pub name: String,
    pub priority: i32,
    pub cluster_targets: Vec<Uuid>,
}

/// DTO for updating an existing queue.
pub struct UpdateQueue {
    pub id: Uuid,
    pub name: String,
    pub priority: i32,
    pub cluster_targets: Vec<Uuid>,
}

pub struct QueueService {
    queue_repo: Arc<dyn QueueRepository>,
}

impl QueueService {
    pub fn new(queue_repo: Arc<dyn QueueRepository>) -> Self {
        Self { queue_repo }
    }

    pub async fn create_queue(&self, new_queue: CreateQueue) -> Result<Queue, anyhow::Error> {
        let queue = Queue {
            id: Uuid::new_v4(),
            name: new_queue.name,
            priority: new_queue.priority,
            cluster_targets: new_queue.cluster_targets,
        };

        self.queue_repo.create(&queue).await?;

        Ok(queue)
    }

    pub async fn get_queue_by_id(&self, id: Uuid) -> Result<Option<Queue>, anyhow::Error> {
        self.queue_repo.find_by_id(id).await
    }

    pub async fn list_all_queues(&self) -> Result<Vec<Queue>, anyhow::Error> {
        self.queue_repo.get_all_queues_sorted().await
    }

    pub async fn update_queue(&self, updated_queue: UpdateQueue) -> Result<Queue, anyhow::Error> {
        let queue = Queue {
            id: updated_queue.id,
            name: updated_queue.name,
            priority: updated_queue.priority,
            cluster_targets: updated_queue.cluster_targets,
        };

        self.queue_repo.update(&queue).await?;

        Ok(queue)
    }

    pub async fn delete_queue(&self, id: Uuid) -> Result<(), anyhow::Error> {
        self.queue_repo.delete(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::queue::ports::MockQueueRepository;
    use mockall::predicate::eq;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_create_queue() {
        let mut mock_repo = MockQueueRepository::new();
        let new_queue_dto = CreateQueue {
            name: "test_queue".to_string(),
            priority: 10,
            cluster_targets: vec![Uuid::new_v4()],
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

        let service = QueueService::new(Arc::new(mock_repo));
        let result = service.create_queue(new_queue_dto).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_queue_by_id_found() {
        let mut mock_repo = MockQueueRepository::new();
        let queue_id = Uuid::new_v4();
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

        let service = QueueService::new(Arc::new(mock_repo));
        let result = service.get_queue_by_id(queue_id).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_get_queue_by_id_not_found() {
        let mut mock_repo = MockQueueRepository::new();
        let queue_id = Uuid::new_v4();

        mock_repo
            .expect_find_by_id()
            .with(eq(queue_id))
            .times(1)
            .returning(|_| Ok(None));

        let service = QueueService::new(Arc::new(mock_repo));
        let result = service.get_queue_by_id(queue_id).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_list_all_queues() {
        let mut mock_repo = MockQueueRepository::new();
        let queues = vec![Queue {
            id: Uuid::new_v4(),
            name: "test".to_string(),
            priority: 1,
            cluster_targets: vec![],
        }];

        mock_repo
            .expect_get_all_queues_sorted()
            .times(1)
            .returning(move || Ok(queues.clone()));

        let service = QueueService::new(Arc::new(mock_repo));
        let result = service.list_all_queues().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_update_queue() {
        let mut mock_repo = MockQueueRepository::new();
        let updated_queue_dto = UpdateQueue {
            id: Uuid::new_v4(),
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

        let service = QueueService::new(Arc::new(mock_repo));
        let result = service.update_queue(updated_queue_dto).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_queue() {
        let mut mock_repo = MockQueueRepository::new();
        let queue_id = Uuid::new_v4();

        mock_repo
            .expect_delete()
            .with(eq(queue_id))
            .times(1)
            .returning(|_| Ok(()));

        let service = QueueService::new(Arc::new(mock_repo));
        let result = service.delete_queue(queue_id).await;

        assert!(result.is_ok());
    }
}