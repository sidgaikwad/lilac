use sqlx::PgPool;

use crate::{
    model::{
        organization::OrganizationId,
        pipeline::{Pipeline, PipelineId},
        step::StepInstance,
    },
    ServiceError,
};

pub async fn get_pipeline(db: &PgPool, pipeline_id: &PipelineId) -> Result<Pipeline, ServiceError> {
    let id = pipeline_id.inner();
    let pipeline = sqlx::query!(
        // language=PostgreSQL
        r#"
            SELECT p.*, ARRAY_AGG((s.step_instance_id, s.step_id, s.pipeline_id, s.previous_step, s.next_step, s.step_parameters)) as "steps: Vec<StepInstance>" 
            FROM "pipelines" p
            RIGHT JOIN "step_instances" s USING (pipeline_id)
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
    .fetch_one(db)
    .await?;
    Ok(pipeline)
}

pub async fn list_pipelines(
    db: &PgPool,
    organization_id: &OrganizationId,
) -> Result<Vec<Pipeline>, ServiceError> {
    let id = organization_id.inner();
    let pipelines = sqlx::query!(
        // language=PostgreSQL
        r#"
            SELECT p.*, ARRAY_AGG((s.*)) as "steps: Vec<StepInstance>" 
            FROM "pipelines" p
            LEFT JOIN "step_instances" s USING (pipeline_id)
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
    .fetch_all(db)
    .await?;
    Ok(pipelines)
}

pub async fn create_pipeline(db: &PgPool, pipeline: Pipeline) -> Result<PipelineId, ServiceError> {
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
    .fetch_one(db)
    .await?;
    Ok(pipeline_id)
}
