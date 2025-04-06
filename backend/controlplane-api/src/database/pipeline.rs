use sqlx::PgPool;

use crate::{
    model::{
        organization::OrganizationId,
        pipeline::{Pipeline, PipelineId},
    },
    ServiceError,
};

pub async fn get_pipeline(db: &PgPool, pipeline_id: &PipelineId) -> Result<Pipeline, ServiceError> {
    let id = pipeline_id.inner();
    let pipeline = sqlx::query_as!(
        Pipeline,
        // language=PostgreSQL
        r#"
            SELECT * FROM "pipelines" WHERE pipeline_id = $1
        "#,
        id
    )
    .fetch_one(db)
    .await?;
    Ok(pipeline)
}

pub async fn list_pipelines(
    db: &PgPool,
    organization_id: &OrganizationId,
) -> Result<Vec<Pipeline>, ServiceError> {
    let id = organization_id.inner();
    let pipelines = sqlx::query_as!(
        Pipeline,
        // language=PostgreSQL
        r#"
            SELECT * FROM "pipelines" WHERE organization_id = $1
        "#,
        id
    )
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
