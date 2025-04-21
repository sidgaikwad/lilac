use std::str::FromStr;

use crate::{
    model::jobs::{Job, JobId, JobStatus},
    ServiceError,
};

use super::Database;

impl Database {
    pub async fn get_pending_job(&self) -> Result<Option<Job>, ServiceError> {
        let job: Option<Result<Job, ServiceError>> = sqlx::query!(
            // language=PostgreSQL
            r#"
                WITH jobs AS MATERIALIZED (
                    SELECT job_id, pipeline_id, status
                        FROM pipeline_jobs
                        WHERE status IN ('pending')
                        ORDER BY created_at
                        LIMIT 1
                        FOR UPDATE SKIP LOCKED
                )
                UPDATE pipeline_jobs
                SET status = 'in_progress', started_at = now()
                WHERE job_id = ANY(SELECT job_id FROM jobs)
                RETURNING job_id, pipeline_id, status;
            "#,
        )
        .map(|row| {
            Ok(Job {
                job_id: row.job_id.into(),
                pipeline_id: row.pipeline_id.into(),
                status: JobStatus::from_str(&row.status).map_err(|_| {
                    ServiceError::ParseError(format!("invalid JobStatus {}", &row.status))
                })?,
            })
        })
        .fetch_optional(&self.pool)
        .await?;
        match job {
            Some(res) => Ok(Some(res?)),
            None => Ok(None),
        }
    }

    pub async fn create_job(&self, job: Job) -> Result<(), ServiceError> {
        sqlx::query!(
            // language=PostgreSQL
            r#"
                INSERT INTO pipeline_jobs
                (job_id, pipeline_id, status)
                VALUES
                ($1, $2, $3)
            "#,
            job.job_id.inner(),
            job.pipeline_id.inner(),
            job.status.to_string(),
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn complete_job(&self, job_id: &JobId) -> Result<(), ServiceError> {
        sqlx::query!(
            // language=PostgreSQL
            r#"
                UPDATE pipeline_jobs
                SET status = 'completed', ended_at = now()
                WHERE job_id = $1
            "#,
            job_id.inner(),
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn fail_job(&self, job_id: &JobId) -> Result<(), ServiceError> {
        sqlx::query!(
            // language=PostgreSQL
            r#"
                UPDATE pipeline_jobs
                SET status = 'failed', ended_at = now()
                WHERE job_id = $1
            "#,
            job_id.inner(),
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
