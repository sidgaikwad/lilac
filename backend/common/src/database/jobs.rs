use std::str::FromStr;

use chrono::Utc;

use crate::{model::jobs::{Job, JobStatus}, ServiceError};

use super::Database;



impl Database {
    pub async fn get_pending_job(
        &self,
    ) -> Result<Option<Job>, ServiceError> {
        let now = Utc::now();
        let job: Option<Result<Job, ServiceError>> = sqlx::query!(
            // language=PostgreSQL
            r#"
                WITH jobs AS MATERIALIZED (
                    SELECT job_id, pipeline_id, status, created_at, started_at, ended_at
                        FROM pipeline_jobs
                        WHERE status IN ('PENDING')
                        ORDER BY created_at
                        LIMIT 1
                        FOR UPDATE SKIP LOCKED
                )
                UPDATE pipeline_jobs
                SET status = 'IN_PROGRESS', started_at = $1
                WHERE job_id = ANY(SELECT job_id FROM jobs)
                RETURNING job_id, pipeline_id, status, created_at, started_at, ended_at;
            "#,
            now,
        )
        .map(|row|
            Ok(Job {
                job_id: row.job_id.into(),
                pipeline_id: row.pipeline_id.into(),
                status: JobStatus::from_str(&row.status).map_err(|_| ServiceError::ParseError(format!("invalid JobStatus {}", &row.status)))?,
                created_at: row.created_at,
                started_at: row.started_at,
                ended_at: row.ended_at,
            })
        )
        .fetch_optional(&self.pool)
        .await?;
        match job {
            Some(res) => Ok(Some(res?)),
            None => Ok(None)
        }
    }

}