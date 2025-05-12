use std::str::FromStr;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    model::jobs::{Job, JobId, JobStatus},
    ServiceError,
};

use super::Database;

#[derive(Debug)]
pub struct JobOutputInfoFromDb {
    pub job_id: JobId,
    pub input_dataset_name: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow, Debug)]
struct JobOutputInfoFromDbSqlx {
    job_id: Uuid,
    input_dataset_name: Option<String>,
    completed_at: Option<DateTime<Utc>>,
}


impl Database {
    pub async fn get_completed_job_outputs_by_project(
        &self,
        project_id_opt: Option<Uuid>,
    ) -> Result<Vec<JobOutputInfoFromDb>, ServiceError> {
        let mut query_builder = sqlx::QueryBuilder::new(
            r#"
            SELECT
                pj.job_id,
                pj.dataset_path AS input_dataset_name,
                pj.ended_at AS completed_at
            FROM
                pipeline_jobs pj
            INNER JOIN
                pipelines p ON pj.pipeline_id = p.pipeline_id
            WHERE
                pj.status = 'completed'
            "#
        );

        if let Some(pid) = project_id_opt {
            query_builder.push(" AND p.project_id = ");
            query_builder.push_bind(pid);
        }

        query_builder.push(" ORDER BY pj.ended_at DESC");
        
        let fetched_rows = query_builder
            .build_query_as::<JobOutputInfoFromDbSqlx>()
            .fetch_all(&self.pool)
            .await?; // Handle Result here

        let results = fetched_rows 
            .into_iter()
            .map(|row| JobOutputInfoFromDb {
                job_id: row.job_id.into(),
                input_dataset_name: row.input_dataset_name,
                completed_at: row.completed_at,
            })
            .collect();

        Ok(results)
    }

    pub async fn get_pending_job(&self) -> Result<Option<Job>, ServiceError> {
        let job: Option<Result<Job, ServiceError>> = sqlx::query!(
            // language=PostgreSQL
            r#"
                WITH jobs AS MATERIALIZED (
                    SELECT job_id, pipeline_id, status, input_dataset_id
                        FROM pipeline_jobs
                        WHERE status IN ('pending')
                        ORDER BY created_at
                        LIMIT 1
                        FOR UPDATE SKIP LOCKED
                )
                UPDATE pipeline_jobs
                SET status = 'in_progress', started_at = now()
                WHERE job_id = ANY(SELECT job_id FROM jobs)
                RETURNING job_id, pipeline_id, status, input_dataset_id;
            "#,
        )
        .map(|row| {
            Ok(Job {
                job_id: row.job_id.into(),
                pipeline_id: row.pipeline_id.into(),
                status: JobStatus::from_str(&row.status).map_err(|_| {
                    ServiceError::ParseError(format!("invalid JobStatus {}", &row.status))
                })?,
                input_dataset_id: row.input_dataset_id.into(),
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
                (job_id, pipeline_id, status, input_dataset_id)
                VALUES
                ($1, $2, $3, $4) -- Added $4
            "#,
            job.job_id.inner(),
            job.pipeline_id.inner(),
            job.status.to_string(),
            job.input_dataset_id.inner(),
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
