use crate::{
    model::{
        pipeline::{Pipeline, PipelineId},
        project::ProjectId,
        step::{Step, StepId},
    },
    ServiceError,
};

use super::Database;

impl Database {
    pub async fn get_pipeline(&self, pipeline_id: &PipelineId) -> Result<Pipeline, ServiceError> {
        let id = pipeline_id.inner();
        let pipeline = sqlx::query!(
            // language=PostgreSQL
            r#"
            SELECT p.pipeline_id, p.pipeline_name, p.description, p.project_id,
                ARRAY(
                    SELECT (s.step_id, s.step_definition_id, s.pipeline_id, s.step_parameters)
                    FROM "steps" s WHERE s.pipeline_id = $1
                ) as "steps: Vec<Step>",
                ARRAY(
                    SELECT (s.from_step_id, s.to_step_id)
                    FROM "step_connections" s WHERE s.pipeline_id = $1
                ) as "step_connections: Vec<(StepId, StepId)>"
            FROM "pipelines" p
            WHERE p.pipeline_id = $1
            GROUP BY p.pipeline_id
        "#,
            id
        )
        .map(|row| Pipeline {
            pipeline_id: row.pipeline_id.into(),
            pipeline_name: row.pipeline_name,
            description: row.description,
            project_id: row.project_id.into(),
            steps: row.steps.unwrap_or(Vec::new()),
            step_connections: row.step_connections.unwrap_or(Vec::new()),
        })
        .fetch_one(&self.pool)
        .await?;
        Ok(pipeline)
    }

    pub async fn list_pipelines(
        &self,
        project_id: &ProjectId,
    ) -> Result<Vec<Pipeline>, ServiceError> {
        let id = project_id.inner();
        let pipelines = sqlx::query!(
            // language=PostgreSQL
            r#"
            SELECT p.pipeline_id, p.pipeline_name, p.description, p.project_id, p.created_at,
                ARRAY_AGG((s.step_id, s.step_definition_id, s.pipeline_id, s.step_parameters)) as "steps: Vec<Step>",
                ARRAY(
                    SELECT (s.from_step_id, s.to_step_id)
                    FROM "step_connections" s WHERE s.pipeline_id = $1
                ) as "step_connections: Vec<(StepId, StepId)>"
            FROM "pipelines" p
            LEFT JOIN "steps" s USING (pipeline_id)
            WHERE p.project_id = $1
            GROUP BY p.pipeline_id
        "#,
            id
        )
        .map(|row| Pipeline {
            pipeline_id: row.pipeline_id.into(),
            pipeline_name: row.pipeline_name,
            description: row.description,
            project_id: row.project_id.into(),
            steps: row.steps.unwrap_or(Vec::new()),
            step_connections: row.step_connections.unwrap_or(Vec::new()),
        })
        .fetch_all(&self.pool)
        .await?;
        Ok(pipelines)
    }

    pub async fn create_pipeline(&self, pipeline: Pipeline) -> Result<PipelineId, ServiceError> {
        let pipeline_id = sqlx::query!(
        // language=PostgreSQL
        r#"
            INSERT INTO "pipelines" (pipeline_id, pipeline_name, description, project_id) VALUES ($1, $2, $3, $4) RETURNING pipeline_id
        "#,
        pipeline.pipeline_id.inner(),
        &pipeline.pipeline_name,
        pipeline.description.as_ref(),
        &pipeline.project_id.inner()
    )
    .map(|row| PipelineId::new(row.pipeline_id))
    .fetch_one(&self.pool)
    .await?;
        Ok(pipeline_id)
    }

    pub async fn delete_pipeline(&self, pipeline_id: &PipelineId) -> Result<(), ServiceError> {
        let id = pipeline_id.inner();
        sqlx::query!(
            // language=PostgreSQL
            r#"
            DELETE FROM pipelines WHERE pipeline_id = $1;
        "#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
