use std::collections::HashSet;

use axum::{extract::{Path, State}, Json};
use common::{
    database::Database,
    model::{
        pipeline::PipelineId,
        step::{Step, StepId},
        step_definition::StepType,
    },
    ServiceError,
};
use jsonschema::is_valid;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::auth::claims::Claims;

#[instrument(level = "info", skip(db), ret, err)]
pub async fn create_step(
    claims: Claims,
    State(db): State<Database>,
    Json(request): Json<CreateStepRequest>,
) -> Result<Json<CreateStepResponse>, ServiceError> {
    let step_definition = db.get_step_definition_by_type(&request.step_type).await?;

    if !is_valid(&step_definition.schema, &request.step_parameters) {
        return Err(ServiceError::SchemaError);
    }

    let step = Step::create(
        step_definition.id,
        request.pipeline_id,
        request.step_parameters,
    );

    let step_id = db.create_step(step).await?;

    Ok(Json(CreateStepResponse { id: step_id }))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStepRequest {
    pipeline_id: PipelineId,
    step_type: StepType,
    step_parameters: serde_json::Value,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStepResponse {
    id: StepId,
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn get_step(
    claims: Claims,
    State(db): State<Database>,
    Path(step_id): Path<StepId>,
) -> Result<Json<GetStepResponse>, ServiceError> {
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
#[serde(rename_all = "camelCase")]
pub struct GetStepResponse {
    step_id: StepId,
    pipeline_id: PipelineId,
    step_type: StepType,
    step_parameters: serde_json::Value,
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn delete_step(
    claims: Claims,
    State(db): State<Database>,
    Path(step_id_param): Path<StepId>,
) -> Result<(), ServiceError> {
    let step = db.get_step(&step_id_param).await?;
    let pipeline = db.get_pipeline(&step.pipeline_id).await?;
    let project = db.get_project(&pipeline.project_id).await?;
    let is_member = db
        .is_user_member_of_organization(&claims.sub, &project.organization_id)
        .await?;
    if !is_member {
        return Err(ServiceError::Unauthorized);
    }

    db.delete_step(&step_id_param).await?;
    Ok(())
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn update_step(
    claims: Claims,
    State(db): State<Database>,
    Path(step_id): Path<StepId>,
    Json(request): Json<UpdateStepRequest>,
) -> Result<(), ServiceError> {
    let step = db.get_step(&step_id).await?;

    if let Some(params) = request.step_parameters {
        let step_definition = db.get_step_definition(&step.step_definition_id).await?;
        if !is_valid(&step_definition.schema, &params) {
            return Err(ServiceError::SchemaError);
        }
        db.update_step(&step_id, &params).await?;
    }
    if let Some(new_connections) = request.connections {
        let new_connections_hs: HashSet<StepId> = HashSet::from_iter(new_connections.clone());

        let curr_connections = db.get_step_connections(&step_id).await?;
        let curr_connections_hs: HashSet<StepId> = HashSet::from_iter(curr_connections.clone());

        let conns_to_delete = curr_connections
            .into_iter()
            .filter(|v| !new_connections_hs.contains(v))
            .collect();
        let conns_to_create = new_connections
            .into_iter()
            .filter(|v| !curr_connections_hs.contains(v))
            .collect();

        db.disconnect_steps(&step.step_id, conns_to_delete).await?;
        db.connect_steps(step.clone(), conns_to_create).await?;
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStepRequest {
    step_parameters: Option<serde_json::Value>,
    connections: Option<Vec<StepId>>,
}
