use axum::{extract::Path, Extension, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::instrument;

use crate::{
    auth::claims::Claims,
    database,
    model::{
        organization::OrganizationId,
        pipeline::{Pipeline, PipelineId},
        step::StepInstance,
    },
    ServiceError,
};

#[instrument(level = "info", skip(db), ret, err)]
pub async fn create_pipeline(
    claims: Claims,
    db: Extension<PgPool>,
    Json(request): Json<CreatePipelineRequest>,
) -> Result<Json<CreatePipelineResponse>, ServiceError> {
    let pipeline = Pipeline::create(request.name, request.description, request.organization_id);

    let pipeline_id = database::create_pipeline(&db, pipeline).await?;

    Ok(Json(CreatePipelineResponse { id: pipeline_id }))
}

#[derive(Debug, Deserialize)]
pub struct CreatePipelineRequest {
    name: String,
    description: Option<String>,
    organization_id: OrganizationId,
}

#[derive(Debug, Serialize)]
pub struct CreatePipelineResponse {
    id: PipelineId,
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn get_pipeline(
    claims: Claims,
    db: Extension<PgPool>,
    Path(pipeline_id): Path<String>,
) -> Result<Json<GetPipelineResponse>, ServiceError> {
    let pipeline_id = PipelineId::try_from(pipeline_id)?;
    let pipeline = database::get_pipeline(&db, &pipeline_id).await?;

    Ok(Json(pipeline.into()))
}

#[derive(Debug, Serialize)]
pub struct GetPipelineResponse {
    id: PipelineId,
    name: String,
    description: Option<String>,
    organization_id: OrganizationId,
    created_at: DateTime<Utc>,
    steps: Vec<StepInstance>,
}

impl From<Pipeline> for GetPipelineResponse {
    fn from(pipeline: Pipeline) -> Self {
        Self {
            id: pipeline.pipeline_id,
            name: pipeline.pipeline_name,
            organization_id: pipeline.organization_id,
            description: pipeline.description,
            created_at: pipeline.created_at,
            steps: pipeline.steps,
        }
    }
}
