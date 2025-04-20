use axum::{extract::Path, Extension, Json};
use common::{
    database::Database,
    model::{
        pipeline::PipelineId,
        step::{Step, StepId},
        step_definition::{StepDefinitionId, StepType},
    },
    ServiceError,
};
use jsonschema::is_valid;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::auth::claims::Claims;

#[instrument(level = "info", skip(db), ret, err)]
pub async fn create_pipeline_step(
    claims: Claims,
    db: Extension<Database>,
    Path(pipeline_id): Path<PipelineId>,
    Json(request): Json<CreateStepInstanceRequest>,
) -> Result<Json<CreateStepInstanceResponse>, ServiceError> {
    let step_definition = db.get_step_definition(&request.step_definition_id).await?;

    if !is_valid(&step_definition.schema, &request.step_parameters) {
        return Err(ServiceError::SchemaError);
    }

    let step = Step::create(
        request.step_definition_id,
        pipeline_id,
        request.step_parameters,
    );

    let step_id = db.create_step(step).await?;

    Ok(Json(CreateStepInstanceResponse { id: step_id }))
}

#[derive(Debug, Deserialize)]
pub struct CreateStepInstanceRequest {
    step_definition_id: StepDefinitionId,
    step_parameters: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct CreateStepInstanceResponse {
    id: StepId,
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn get_pipeline_step(
    claims: Claims,
    db: Extension<Database>,
    Path((pipeline_id, step_id)): Path<(PipelineId, StepId)>,
) -> Result<Json<GetStepResponse>, ServiceError> {
    let _pipeline = db.get_pipeline(&pipeline_id).await?;
    let step = db.get_step(&step_id).await?;
    let step_definition = db.get_step_definition(&step.step_definition_id).await?;
    Ok(Json(GetStepResponse {
        step_id: step.step_id,
        pipeline_id: step.pipeline_id,
        step_type: step_definition.step_type,
        step_parameters: step.step_parameters,
    }))
}

#[derive(Debug, Serialize)]
pub struct GetStepResponse {
    step_id: StepId,
    pipeline_id: PipelineId,
    step_type: StepType,
    step_parameters: serde_json::Value,
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn connect_pipeline_step(
    claims: Claims,
    db: Extension<Database>,
    Path((pipeline_id, from_step_id, to_step_id)): Path<(PipelineId, StepId, StepId)>,
) -> Result<(), ServiceError> {
    let _pipeline = db.get_pipeline(&pipeline_id).await?;
    let from_step = db.get_step(&from_step_id).await?;
    let to_step = db.get_step(&to_step_id).await?;
    if from_step.pipeline_id == to_step.pipeline_id {
        db.connect_steps(from_step, to_step).await?;
        Ok(())
    } else {
        Err(ServiceError::Unauthorized)
    }
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn disconnect_pipeline_step(
    claims: Claims,
    db: Extension<Database>,
    Path((pipeline_id, from_step_id, to_step_id)): Path<(PipelineId, StepId, StepId)>,
) -> Result<(), ServiceError> {
    let _pipeline = db.get_pipeline(&pipeline_id).await?;
    let from_step = db.get_step(&from_step_id).await?;
    let to_step = db.get_step(&to_step_id).await?;
    if from_step.pipeline_id == to_step.pipeline_id {
        db.disconnect_steps(from_step, to_step).await?;
        Ok(())
    } else {
        Err(ServiceError::Unauthorized)
    }
}
