use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    cluster::models::NodeId,
    queue::models::QueueId,
    training_job::{
        models::{GetTrainingJobsFilters, JobId, TrainingJob, TrainingJobStatus},
        ports::{TrainingJobRepository, TrainingJobRepositoryError},
    },
};

pub struct PostgresTrainingJobRepository {
    pool: PgPool,
}

impl PostgresTrainingJobRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "training_job_status", rename_all = "lowercase")]
pub(super) enum TrainingJobStatusRecord {
    Queued,
    Starting,
    Running,
    Succeeded,
    Failed,
}

impl From<TrainingJobStatus> for TrainingJobStatusRecord {
    fn from(value: TrainingJobStatus) -> Self {
        match value {
            TrainingJobStatus::Queued => Self::Queued,
            TrainingJobStatus::Starting => Self::Starting,
            TrainingJobStatus::Running => Self::Running,
            TrainingJobStatus::Succeeded => Self::Succeeded,
            TrainingJobStatus::Failed => Self::Failed,
        }
    }
}

impl From<TrainingJobStatusRecord> for TrainingJobStatus {
    fn from(value: TrainingJobStatusRecord) -> Self {
        match value {
            TrainingJobStatusRecord::Queued => Self::Queued,
            TrainingJobStatusRecord::Starting => Self::Starting,
            TrainingJobStatusRecord::Running => Self::Running,
            TrainingJobStatusRecord::Succeeded => Self::Succeeded,
            TrainingJobStatusRecord::Failed => Self::Failed,
        }
    }
}

#[derive(sqlx::FromRow)]
pub(super) struct TrainingJobRecord {
    pub(super) id: Uuid,
    pub(super) name: String,
    pub(super) definition: String,
    pub(super) status: TrainingJobStatusRecord,
    pub(super) node_id: Option<Uuid>,
    pub(super) queue_id: Uuid,
    pub(super) resource_requirements: serde_json::Value,
    pub(super) created_at: chrono::DateTime<chrono::Utc>,
    pub(super) updated_at: chrono::DateTime<chrono::Utc>,
}

impl TryFrom<TrainingJobRecord> for TrainingJob {
    type Error = anyhow::Error;

    fn try_from(value: TrainingJobRecord) -> Result<Self, Self::Error> {
        let resource_requirements = serde_json::from_value(value.resource_requirements)?;
        Ok(Self {
            id: value.id.into(),
            name: value.name,
            definition: value.definition,
            status: value.status.into(),
            node_id: value.node_id.map(|v| v.into()),
            queue_id: value.queue_id.into(),
            resource_requirements: resource_requirements,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}

#[async_trait]
impl TrainingJobRepository for PostgresTrainingJobRepository {
    async fn create(&self, training_job: &TrainingJob) -> Result<(), TrainingJobRepositoryError> {
        sqlx::query!(
            "INSERT INTO training_jobs (id, name, definition, status, queue_id, resource_requirements, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            training_job.id.0,
            training_job.name,
            training_job.definition,
            TrainingJobStatusRecord::from(training_job.status.clone()) as _,
            training_job.queue_id.0,
            &serde_json::to_value(&training_job.resource_requirements).map_err(|e| anyhow::anyhow!(e))?,
            training_job.created_at,
            training_job.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e: sqlx::Error| TrainingJobRepositoryError::Unknown(anyhow::anyhow!(e)))?;

        Ok(())
    }

    async fn get_training_jobs(
        &self,
        filters: GetTrainingJobsFilters,
    ) -> Result<Vec<TrainingJob>, TrainingJobRepositoryError> {
        let mut query = sqlx::QueryBuilder::new(
            "SELECT id, name, definition, status, instance_id, queue_id, resource_requirements, scheduled_cluster_id, created_at, updated_at FROM training_jobs WHERE 1 = 1"
        );

        if let Some(id) = filters.id {
            query.push(" AND id = ");
            query.push_bind(id);
        }

        if let Some(name) = filters.name {
            query.push(" AND name = ");
            query.push_bind(name);
        }

        if let Some(status) = filters.status {
            query.push(" AND status = ");
            query.push_bind(TrainingJobStatusRecord::from(status));
        }

        let rows: Vec<TrainingJobRecord> = query
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(|e: sqlx::Error| TrainingJobRepositoryError::Unknown(anyhow::anyhow!(e)))?;

        let training_jobs = rows
            .into_iter()
            .map(|row| row.try_into())
            .collect::<Result<Vec<TrainingJob>, anyhow::Error>>()?;

        Ok(training_jobs)
    }

    async fn update_status(
        &self,
        id: &JobId,
        status: TrainingJobStatus,
    ) -> Result<(), TrainingJobRepositoryError> {
        sqlx::query!(
            "UPDATE training_jobs SET status = $1 WHERE id = $2",
            TrainingJobStatusRecord::from(status) as _,
            id.0
        )
        .execute(&self.pool)
        .await
        .map_err(|e: sqlx::Error| TrainingJobRepositoryError::Unknown(anyhow::anyhow!(e)))?;

        Ok(())
    }

    async fn mark_as_starting(
        &self,
        id: &JobId,
        node_id: &NodeId,
    ) -> Result<(), TrainingJobRepositoryError> {
        sqlx::query!(
            "UPDATE training_jobs SET status = 'starting', node_id = $1 WHERE id = $2",
            node_id.0,
            id.0
        )
        .execute(&self.pool)
        .await
        .map_err(|e: sqlx::Error| TrainingJobRepositoryError::Unknown(anyhow::anyhow!(e)))?;

        Ok(())
    }

    async fn get_queued_jobs_for_queue(
        &self,
        queue_id: &QueueId,
    ) -> Result<Vec<TrainingJob>, TrainingJobRepositoryError> {
        let rows = sqlx::query_as!(
            TrainingJobRecord,
            r#"
            SELECT
                id, name, definition, status AS "status: TrainingJobStatusRecord", node_id, queue_id,
                resource_requirements, created_at, updated_at
            FROM training_jobs
            WHERE status = 'queued' AND queue_id = $1
            ORDER BY created_at ASC
            "#,
            queue_id.0,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e: sqlx::Error| TrainingJobRepositoryError::Unknown(anyhow::anyhow!(e)))?;

        let jobs = rows
            .into_iter()
            .map(|row| row.try_into())
            .collect::<Result<Vec<_>, anyhow::Error>>()?;

        Ok(jobs)
    }
    async fn post_logs(
        &self,
        _id: &JobId,
        _logs: String,
    ) -> Result<(), TrainingJobRepositoryError> {
        // TODO: Implement log ingestion. This could involve writing to a file,
        // a separate logging service, or another table.
        todo!("TODO: Implement log posting");
    }
}
