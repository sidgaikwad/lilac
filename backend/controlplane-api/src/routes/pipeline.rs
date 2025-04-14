use axum::{extract::Path, Extension, Json};
use chrono::{DateTime, Utc};
use common::{
    database::Database,
    model::{
        organization::OrganizationId,
        pipeline::{Pipeline, PipelineId},
        step::Step,
    },
    ServiceError,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::auth::claims::Claims;

#[instrument(level = "info", skip(db), ret, err)]
pub async fn create_pipeline(
    claims: Claims,
    db: Extension<Database>,
    Json(request): Json<CreatePipelineRequest>,
) -> Result<Json<CreatePipelineResponse>, ServiceError> {
    let pipeline = Pipeline::create(request.name, request.description, request.organization_id);

    let pipeline_id = db.create_pipeline(pipeline).await?;

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
    db: Extension<Database>,
    Path(pipeline_id): Path<String>,
) -> Result<Json<GetPipelineResponse>, ServiceError> {
    let pipeline_id = PipelineId::try_from(pipeline_id)?;
    let pipeline = db.get_pipeline(&pipeline_id).await?;

    Ok(Json(pipeline.into()))
}

#[derive(Debug, Serialize)]
pub struct GetPipelineResponse {
    id: PipelineId,
    name: String,
    description: Option<String>,
    organization_id: OrganizationId,
    created_at: DateTime<Utc>,
    steps: Vec<Step>,
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
