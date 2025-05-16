use axum::{extract::{Path, State}, Json};
use common::{
    database::Database,
    model::{
        dataset::DatasetId, jobs::{Job, JobId}, pipeline::{Pipeline, PipelineId}, project::ProjectId, step::{Step, StepId}, step_definition::StepDefinitionId
    },
    ServiceError,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use validator::Validate;

use crate::auth::claims::Claims;

#[instrument(level = "info", skip(db), ret, err)]
pub async fn create_pipeline(
    claims: Claims,
    State(db): State<Database>,
    Json(request): Json<CreatePipelineRequest>,
) -> Result<Json<CreatePipelineResponse>, ServiceError> {
    match request.validate() {
        Ok(_) => (),
        Err(e) => return Err(ServiceError::SchemaValidationError(e.to_string())),
    }
    let pipeline = Pipeline::create(request.name, request.description, request.project_id);

    let pipeline_id = db.create_pipeline(pipeline).await?;

    Ok(Json(CreatePipelineResponse { id: pipeline_id }))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreatePipelineRequest {
    #[validate(length(min = 1, message = "Pipeline name cannot be empty"))]
    name: String,
    #[validate(length(min = 1, message = "Description cannot be empty if provided"))]
    description: Option<String>,
    project_id: ProjectId,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePipelineResponse {
    id: PipelineId,
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn get_pipeline(
    claims: Claims,
    State(db): State<Database>,
    Path(pipeline_id): Path<String>,
) -> Result<Json<GetPipelineResponse>, ServiceError> {
    let pipeline_id = PipelineId::try_from(pipeline_id)?;
    let pipeline = db.get_pipeline(&pipeline_id).await?;

    Ok(Json(pipeline.into()))
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPipelineResponse {
    id: PipelineId,
    name: String,
    description: Option<String>,
    project_id: ProjectId,
    steps: Vec<Step>,
    step_connections: Vec<(StepId, StepId)>,
}

impl From<Pipeline> for GetPipelineResponse {
    fn from(pipeline: Pipeline) -> Self {
        Self {
            id: pipeline.pipeline_id,
            name: pipeline.pipeline_name,
            project_id: pipeline.project_id,
            description: pipeline.description,
            steps: pipeline.steps,
            step_connections: pipeline.step_connections,
        }
    }
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn run_pipeline(
    claims: Claims,
    State(db): State<Database>,
    Path(pipeline_id): Path<String>,
    Json(request): Json<RunPipelineApiRequest>,
) -> Result<Json<RunPipelineResponse>, ServiceError> {
    let pipeline_id = PipelineId::try_from(pipeline_id)?;
    let pipeline = db.get_pipeline(&pipeline_id).await?;

    let job = Job::create(pipeline.pipeline_id, request.dataset_id);
    let job_id = job.job_id.clone();
    db.create_job(job).await?;

    Ok(Json(RunPipelineResponse { id: job_id }))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunPipelineApiRequest {
    pub dataset_id: DatasetId,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunPipelineResponse {
    id: JobId,
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn delete_pipeline(
    claims: Claims,
    State(db): State<Database>,
    Path(pipeline_id_str): Path<String>,
) -> Result<(), ServiceError> {
    let pipeline_id = PipelineId::try_from(pipeline_id_str)?;

    let pipeline = db.get_pipeline(&pipeline_id).await?;
    let project = db.get_project(&pipeline.project_id).await?;
    let is_member = db
        .is_user_member_of_organization(&claims.sub, &project.organization_id)
        .await?;
    if !is_member {
        return Err(ServiceError::Unauthorized);
    }

    db.delete_pipeline(&pipeline_id).await?;

    Ok(())
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn update_pipeline(
    claims: Claims,
    State(db): State<Database>,
    Path(pipeline_id): Path<String>,
    Json(request): Json<UpdatePipelineRequest>,
) -> Result<(), ServiceError> {
    match request.validate() {
        Ok(_) => (),
        Err(e) => return Err(ServiceError::SchemaValidationError(e.to_string())),
    }
    let pipeline_id = PipelineId::try_from(pipeline_id)?;
    let mut pipeline = db.get_pipeline(&pipeline_id).await?;

    if let Some(new_pipeline_name) = request.pipeline_name {
        pipeline.pipeline_name = new_pipeline_name;
    }
    if let Some(new_description) = request.description {
        match new_description.as_str() {
            "" => pipeline.description = None,
            _ => pipeline.description = Some(new_description),
        }
    }

    if let Some(new_steps) = request.steps {
        pipeline.steps = new_steps
            .into_iter()
            .map(|s| {
                Step::new(
                    s.step_id,
                    s.step_definition_id,
                    pipeline.pipeline_id.clone(),
                    s.step_parameters,
                )
            })
            .collect();
    }

    if let Some(new_step_connections) = request.step_connections {
        pipeline.step_connections = new_step_connections;
    }

    db.update_pipeline(pipeline).await?;

    Ok(())
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePipelineRequest {
    #[validate(length(min = 1, message = "Pipeline name cannot be empty if provided"))]
    pub pipeline_name: Option<String>,
    #[validate(length(min = 1, message = "Description cannot be empty if provided"))]
    pub description: Option<String>,
    pub steps: Option<Vec<UpdateStepRequest>>,
    pub step_connections: Option<Vec<(StepId, StepId)>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStepRequest {
    pub step_id: StepId,
    pub step_definition_id: StepDefinitionId,
    pub step_parameters: serde_json::Value,
}
