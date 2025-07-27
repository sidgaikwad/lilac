use std::sync::Arc;

use uuid::Uuid;

use super::{
    models::{
        GetTrainingJobsFilters, TrainingJob, TrainingJobClusterTarget, TrainingJobStatus,
        TrainingJobWithTargets,
    },
    ports::{TrainingJobRepository, TrainingJobService},
};
use crate::inbound::http::routes::training_jobs::models::CreateTrainingJobRequest;
use async_trait::async_trait;

pub struct TrainingJobServiceImpl {
    repository: Arc<dyn TrainingJobRepository>,
}

impl TrainingJobServiceImpl {
    pub fn new(repository: Arc<dyn TrainingJobRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl TrainingJobService for TrainingJobServiceImpl {
    async fn create(
        &self,
        request: CreateTrainingJobRequest,
    ) -> Result<TrainingJobWithTargets, anyhow::Error> {
        let job_id = Uuid::new_v4();
        let now = chrono::Utc::now();

        let training_job = TrainingJob {
            id: job_id,
            name: request.name,
            definition: request.definition,
            status: TrainingJobStatus::Queued,
            instance_id: None,
            priority: request.priority,
            resource_requirements: serde_json::from_value(request.resource_requirements)?,
            scheduled_cluster_id: None,
            created_at: now,
            updated_at: now,
        };

        let targets: Vec<TrainingJobClusterTarget> = request
            .targets
            .into_iter()
            .map(|t| TrainingJobClusterTarget {
                job_id,
                cluster_id: t.cluster_id,
                priority: t.priority,
            })
            .collect();

        self.repository.create(&training_job, &targets).await?;

        Ok(TrainingJobWithTargets {
            job: training_job,
            targets,
        })
    }

    async fn get_training_jobs(
        &self,
        filters: GetTrainingJobsFilters,
    ) -> Result<Vec<TrainingJob>, anyhow::Error> {
        self.repository.get_training_jobs(filters).await
    }

    async fn update_status(&self, id: Uuid, status: TrainingJobStatus) -> Result<(), anyhow::Error> {
        self.repository.update_status(id, status).await
    }

    async fn mark_as_starting(&self, id: Uuid, cluster_id: Uuid) -> Result<(), anyhow::Error> {
        self.repository.mark_as_starting(id, cluster_id).await
    }

    async fn schedule(&self, id: Uuid) -> Result<(), anyhow::Error> {
        // TODO: Implement scheduling logic
        self.repository.schedule(id).await
    }

    async fn post_logs(&self, id: Uuid, logs: String) -> Result<(), anyhow::Error> {
        self.repository.post_logs(id, logs).await
    }
}
