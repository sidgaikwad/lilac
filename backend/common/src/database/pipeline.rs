use crate::{
    model::{
        organization::OrganizationId,
        pipeline::{Pipeline, PipelineId},
        step::Step,
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
            SELECT p.pipeline_id, p.pipeline_name, p.description, p.organization_id, p.created_at,
                ARRAY(
                    SELECT (s.step_id, s.step_definition_id, s.pipeline_id, s.step_parameters)
                    FROM "steps" s WHERE s.pipeline_id = $1
                ) as "steps: Vec<Step>"
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
            organization_id: row.organization_id.into(),
            steps: row.steps.unwrap_or(Vec::new()),
            created_at: row.created_at,
        })
        .fetch_one(&self.pool)
        .await?;
        Ok(pipeline)
    }

    pub async fn list_pipelines(
        &self,
        organization_id: &OrganizationId,
    ) -> Result<Vec<Pipeline>, ServiceError> {
        let id = organization_id.inner();
        let pipelines = sqlx::query!(
            // language=PostgreSQL
            r#"
            SELECT p.pipeline_id, p.pipeline_name, p.description, p.organization_id, p.created_at,
                ARRAY_AGG((s.step_id, s.step_definition_id, s.pipeline_id, s.step_parameters)) as "steps: Vec<Step>"
            FROM "pipelines" p
            LEFT JOIN "steps" s USING (pipeline_id)
            WHERE p.organization_id = $1
            GROUP BY p.pipeline_id
        "#,
            id
        )
        .map(|row| Pipeline {
            pipeline_id: row.pipeline_id.into(),
            pipeline_name: row.pipeline_name,
            description: row.description,
            organization_id: row.organization_id.into(),
            steps: row.steps.unwrap_or(Vec::new()),
            created_at: row.created_at,
        })
        .fetch_all(&self.pool)
        .await?;
        Ok(pipelines)
    }

    pub async fn create_pipeline(&self, pipeline: Pipeline) -> Result<PipelineId, ServiceError> {
        let pipeline_id = sqlx::query!(
        // language=PostgreSQL
        r#"
            INSERT INTO "pipelines" (pipeline_id, pipeline_name, description, organization_id, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING pipeline_id
        "#,
        pipeline.pipeline_id.inner(),
        &pipeline.pipeline_name,
        pipeline.description.as_ref(),
        &pipeline.organization_id.inner(),
        &pipeline.created_at,
    )
    .map(|row| PipelineId::new(row.pipeline_id))
    .fetch_one(&self.pool)
    .await?;
        Ok(pipeline_id)
    }
}
