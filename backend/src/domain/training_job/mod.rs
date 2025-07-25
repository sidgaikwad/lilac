pub mod models;
pub mod ports;
pub mod service;

#[cfg(test)]
mod tests {
    use super::{
        models::{GetTrainingJobsFilters, TrainingJob, TrainingJobStatus},
        ports::MockTrainingJobRepository,
        service::TrainingJobServiceImpl,
    };
    use crate::domain::training_job::ports::TrainingJobService;
    use std::sync::Arc;
    use uuid::Uuid;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_create_training_job() {
        let mut mock_repo = MockTrainingJobRepository::new();
        let cluster_id = Uuid::new_v4();

        mock_repo
            .expect_create()
            .withf(|_| true)
            .times(1)
            .returning(|_| Ok(()));

        let service = TrainingJobServiceImpl::new(Arc::new(mock_repo));
        let result = service
            .create(
                "test".to_string(),
                "definition".to_string(),
                cluster_id,
            )
            .await;

        assert!(result.is_ok());
        let training_job = result.unwrap();
        assert_eq!(training_job.name, "test");
        assert_eq!(training_job.definition, "definition");
        assert_eq!(training_job.cluster_id, cluster_id);
        assert_eq!(training_job.status, TrainingJobStatus::Queued);
    }


    #[tokio::test]
    async fn test_get_training_jobs() {
        let mut mock_repo = MockTrainingJobRepository::new();
        let filters = GetTrainingJobsFilters {
            status: Some(TrainingJobStatus::Queued),
            ..Default::default()
        };

        mock_repo
            .expect_get_training_jobs()
            .with(eq(filters.clone()))
            .times(1)
            .returning(|_| Ok(vec![]));

        let service = TrainingJobServiceImpl::new(Arc::new(mock_repo));
        let result = service.get_training_jobs(filters).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_status() {
        let mut mock_repo = MockTrainingJobRepository::new();
        let id = Uuid::new_v4();
        let status = TrainingJobStatus::Running;

        mock_repo
            .expect_update_status()
            .with(eq(id), eq(status.clone()))
            .times(1)
            .returning(|_, _| Ok(()));

        let service = TrainingJobServiceImpl::new(Arc::new(mock_repo));
        let result = service.update_status(id, status).await;

        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_schedule() {
        let mut mock_repo = MockTrainingJobRepository::new();
        let id = Uuid::new_v4();

        mock_repo
            .expect_schedule()
            .with(eq(id))
            .times(1)
            .returning(|_| Ok(()));

        let service = TrainingJobServiceImpl::new(Arc::new(mock_repo));
        let result = service.schedule(id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_post_logs() {
        let mut mock_repo = MockTrainingJobRepository::new();
        let id = Uuid::new_v4();

        mock_repo
            .expect_post_logs()
            .with(eq(id), eq("logs".to_string()))
            .times(1)
            .returning(|_, _| Ok(()));

        let service = TrainingJobServiceImpl::new(Arc::new(mock_repo));
        let result = service.post_logs(id, "logs".to_string()).await;

        assert!(result.is_ok());
    }
}
