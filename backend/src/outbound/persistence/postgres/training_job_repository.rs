use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::training_job::{
    models::{GetTrainingJobsFilters, TrainingJob, TrainingJobStatus},
    ports::TrainingJobRepository,
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
            "INSERT INTO training_jobs (id, name, definition, status, cluster_id, instance_id, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            training_job.id,
            training_job.name,
            training_job.definition,
            training_job.status as _,
            training_job.cluster_id,
            training_job.instance_id,
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
        let mut query = sqlx::QueryBuilder::new(
            "SELECT id, name, definition, status, cluster_id, instance_id, created_at, updated_at FROM training_jobs WHERE 1 = 1"
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

        if let Some(cluster_id) = filters.cluster_id {
            query.push(" AND cluster_id = ");
            query.push_bind(cluster_id);
        }

        let training_jobs = query
            .build_query_as()
            .fetch_all(&self.pool)
            .await?;

        Ok(training_jobs)
    }

    async fn update_status(&self, id: Uuid, status: TrainingJobStatus) -> Result<(), anyhow::Error> {
        sqlx::query!(
            "UPDATE training_jobs SET status = $1, updated_at = $2 WHERE id = $3",
            status as _,
            chrono::Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    async fn schedule(&self, id: Uuid) -> Result<(), anyhow::Error> {
        // TODO: Implement schedule logic in db
        Ok(())
    }

    async fn post_logs(&self, id: Uuid, logs: String) -> Result<(), anyhow::Error> {
        // TODO: Implement log ingestion in db
        Ok(())
    }
}
