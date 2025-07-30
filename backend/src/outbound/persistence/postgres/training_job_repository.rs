use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    cluster::models::ClusterId,
    queue::models::QueueId,
    training_job::{
        models::{GetTrainingJobsFilters, JobId, TrainingJob, TrainingJobStatus},
        ports::TrainingJobRepository,
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

#[async_trait]
impl TrainingJobRepository for PostgresTrainingJobRepository {
    async fn create(&self, training_job: &TrainingJob) -> Result<(), anyhow::Error> {
        sqlx::query!(
            "INSERT INTO training_jobs (id, name, definition, status, queue_id, resource_requirements, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            training_job.id.0,
            training_job.name,
            training_job.definition,
            training_job.status as _,
            training_job.queue_id.0,
            &serde_json::to_value(&training_job.resource_requirements)?,
            training_job.created_at,
            training_job.updated_at,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_training_jobs(
        &self,
        filters: GetTrainingJobsFilters,
    ) -> Result<Vec<TrainingJob>, anyhow::Error> {
        #[derive(sqlx::FromRow)]
        struct TrainingJobRow {
            id: Uuid,
            name: String,
            definition: String,
            status: TrainingJobStatus,
            instance_id: Option<Uuid>,
            queue_id: Uuid,
            resource_requirements: serde_json::Value,
            scheduled_cluster_id: Option<Uuid>,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }

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
            query.push_bind(status as TrainingJobStatus);
        }

        let rows: Vec<TrainingJobRow> = query.build_query_as().fetch_all(&self.pool).await?;

        let training_jobs = rows
            .into_iter()
            .map(|row| {
                let resource_requirements = serde_json::from_value(row.resource_requirements)
                    .map_err(|e| anyhow::anyhow!("Failed to parse resource_requirements: {}", e))?;

                Ok(TrainingJob {
                    id: row.id.into(),
                    name: row.name,
                    definition: row.definition,
                    status: row.status,
                    queue_id: row.queue_id.into(),
                    resource_requirements,
                    scheduled_cluster_id: row.scheduled_cluster_id.map(|v| v.into()),
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                })
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?;

        Ok(training_jobs)
    }

    async fn update_status(
        &self,
        id: &JobId,
        status: TrainingJobStatus,
    ) -> Result<(), anyhow::Error> {
        sqlx::query!(
            "UPDATE training_jobs SET status = $1 WHERE id = $2",
            status as _,
            id.0
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn mark_as_starting(
        &self,
        id: &JobId,
        cluster_id: &ClusterId,
    ) -> Result<(), anyhow::Error> {
        sqlx::query!(
            "UPDATE training_jobs SET status = 'starting', scheduled_cluster_id = $1 WHERE id = $2",
            cluster_id.0,
            id.0
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_queued_jobs_for_queue(
        &self,
        queue_id: &QueueId,
    ) -> Result<Vec<TrainingJob>, anyhow::Error> {
        #[derive(sqlx::FromRow)]
        struct QueuedJobRow {
            id: Uuid,
            name: String,
            definition: String,
            status: TrainingJobStatus,
            instance_id: Option<Uuid>,
            queue_id: Uuid,
            resource_requirements: serde_json::Value,
            scheduled_cluster_id: Option<Uuid>,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }

        let rows = sqlx::query_as!(
            QueuedJobRow,
            r#"
            SELECT
                id, name, definition, status AS "status: _", instance_id, queue_id,
                resource_requirements, scheduled_cluster_id, created_at, updated_at
            FROM training_jobs
            WHERE status = 'queued' AND queue_id = $1
            ORDER BY created_at ASC
            "#,
            queue_id.0,
        )
        .fetch_all(&self.pool)
        .await?;

        let jobs = rows
            .into_iter()
            .map(|row| {
                let resource_requirements = serde_json::from_value(row.resource_requirements)
                    .map_err(|e| anyhow::anyhow!("Failed to parse resource_requirements: {}", e))?;

                Ok(TrainingJob {
                    id: row.id.into(),
                    name: row.name,
                    definition: row.definition,
                    status: row.status,
                    queue_id: row.queue_id.into(),
                    resource_requirements,
                    scheduled_cluster_id: row.scheduled_cluster_id.map(|v| v.into()),
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                })
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?;

        Ok(jobs)
    }
    async fn post_logs(&self, _id: &JobId, _logs: String) -> Result<(), anyhow::Error> {
        // TODO: Implement log ingestion. This could involve writing to a file,
        // a separate logging service, or another table.
        println!("TODO: Implement log posting");
        Ok(())
    }
}
