use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::training_job::{
    models::{
        GetTrainingJobsFilters, TrainingJob, TrainingJobClusterTarget, TrainingJobStatus,
        TrainingJobWithTargets,
    },
    ports::TrainingJobRepository,
};
use std::collections::HashMap;

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
    async fn create(
        &self,
        training_job: &TrainingJob,
        targets: &[TrainingJobClusterTarget],
    ) -> Result<(), anyhow::Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!(
            "INSERT INTO training_jobs (id, name, definition, status, instance_id, priority, resource_requirements, scheduled_cluster_id, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
            training_job.id,
            training_job.name,
            training_job.definition,
            training_job.status as _,
            training_job.instance_id,
            training_job.priority,
            &serde_json::to_value(&training_job.resource_requirements)?,
            training_job.scheduled_cluster_id,
            training_job.created_at,
            training_job.updated_at,
        )
        .execute(&mut *tx)
        .await?;

        for target in targets {
            sqlx::query!(
                "INSERT INTO training_job_cluster_targets (job_id, cluster_id, priority)
                 VALUES ($1, $2, $3)",
                target.job_id,
                target.cluster_id,
                target.priority
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

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
            priority: i32,
            resource_requirements: serde_json::Value,
            scheduled_cluster_id: Option<Uuid>,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }

        let mut query = sqlx::QueryBuilder::new(
            "SELECT id, name, definition, status, instance_id, priority, resource_requirements, scheduled_cluster_id, created_at, updated_at FROM training_jobs WHERE 1 = 1"
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

        let rows: Vec<TrainingJobRow> = query
            .build_query_as()
            .fetch_all(&self.pool)
            .await?;

        let training_jobs = rows
            .into_iter()
            .map(|row| {
                let resource_requirements = serde_json::from_value(row.resource_requirements)
                    .map_err(|e| anyhow::anyhow!("Failed to parse resource_requirements: {}", e))?;

                Ok(TrainingJob {
                    id: row.id,
                    name: row.name,
                    definition: row.definition,
                    status: row.status,
                    instance_id: row.instance_id,
                    priority: row.priority,
                    resource_requirements,
                    scheduled_cluster_id: row.scheduled_cluster_id,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                })
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?;

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

    async fn mark_as_starting(&self, id: Uuid, cluster_id: Uuid) -> Result<(), anyhow::Error> {
        sqlx::query!(
            "UPDATE training_jobs SET status = 'starting', scheduled_cluster_id = $1, updated_at = $2 WHERE id = $3",
            cluster_id,
            chrono::Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_queued_jobs_with_targets(
        &self,
    ) -> Result<Vec<TrainingJobWithTargets>, anyhow::Error> {
        #[derive(sqlx::FromRow, Debug)]
        struct QueuedJobRow {
            job_id: Uuid,
            name: String,
            definition: String,
            status: TrainingJobStatus,
            instance_id: Option<Uuid>,
            priority: i32,
            resource_requirements: serde_json::Value,
            scheduled_cluster_id: Option<Uuid>,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
            target_cluster_id: Option<Uuid>,
            target_priority: Option<i32>,
        }

        let rows = sqlx::query_as!(
            QueuedJobRow,
            r#"
            SELECT
                tj.id as "job_id",
                tj.name,
                tj.definition,
                tj.status AS "status: _",
                tj.instance_id,
                tj.priority,
                tj.resource_requirements,
                tj.scheduled_cluster_id,
                tj.created_at,
                tj.updated_at,
                tct.cluster_id as "target_cluster_id",
                tct.priority as "target_priority"
            FROM
                training_jobs tj
            LEFT JOIN
                training_job_cluster_targets tct ON tj.id = tct.job_id
            WHERE
                tj.status = 'queued'
            ORDER BY
                tj.priority, tct.priority
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut jobs_map: HashMap<Uuid, TrainingJobWithTargets> = HashMap::new();

        for row in rows {
            let job_entry = jobs_map.entry(row.job_id).or_insert_with_key(|job_id| {
                let resource_requirements = serde_json::from_value(row.resource_requirements.clone())
                    .unwrap_or_else(|e| {
                        // Log the error and provide a default value
                        tracing::error!(
                            "Failed to parse resource_requirements for job {}: {}. Using default.",
                            job_id,
                            e
                        );
                        crate::domain::training_job::models::ResourceRequirements {
                            cpu_millicores: 0,
                            memory_mb: 0,
                            gpus: None,
                        }
                    });

                TrainingJobWithTargets {
                    job: TrainingJob {
                        id: row.job_id,
                        name: row.name.clone(),
                        definition: row.definition.clone(),
                        status: row.status.clone(),
                        instance_id: row.instance_id,
                        priority: row.priority,
                        resource_requirements,
                        scheduled_cluster_id: row.scheduled_cluster_id,
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                    },
                    targets: Vec::new(),
                }
            });

            if let (Some(cluster_id), Some(priority)) = (row.target_cluster_id, row.target_priority) {
                job_entry.targets.push(TrainingJobClusterTarget {
                    job_id: row.job_id,
                    cluster_id,
                    priority,
                });
            }
        }

        Ok(jobs_map.into_values().collect())
    }

}
