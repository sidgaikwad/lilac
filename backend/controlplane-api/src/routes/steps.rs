use axum::{extract::Path, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::instrument;

use crate::{
    auth::claims::Claims,
    database,
    model::{
        pipeline::PipelineId,
        step::{StepId, StepInstance, StepInstanceId},
    },
    ServiceError,
};

#[instrument(level = "info", skip(db), ret, err)]
pub async fn create_pipeline_step(
    claims: Claims,
    db: Extension<PgPool>,
    Path(pipeline_id): Path<PipelineId>,
    Json(request): Json<CreateStepInstanceRequest>,
) -> Result<Json<CreateStepInstanceResponse>, ServiceError> {
    let step_instance = StepInstance::create(request.step_id, pipeline_id, request.step_parameters);

    let step_instance_id = database::create_step_instance(&db, step_instance).await?;

    Ok(Json(CreateStepInstanceResponse {
        id: step_instance_id,
    }))
}

#[derive(Debug, Deserialize)]
pub struct CreateStepInstanceRequest {
    step_id: StepId,
    step_parameters: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct CreateStepInstanceResponse {
    id: StepInstanceId,
}
