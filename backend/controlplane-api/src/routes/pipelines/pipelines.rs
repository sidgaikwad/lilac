use axum::{extract::Path, Extension, Json};
use common::{
    database::Database,
    model::{
        jobs::{Job, JobId},
        pipeline::{Pipeline, PipelineId},
        project::ProjectId,
        step::{Step, StepId},
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
    let pipeline = Pipeline::create(request.name, request.description, request.project_id);

    let pipeline_id = db.create_pipeline(pipeline).await?;

    Ok(Json(CreatePipelineResponse { id: pipeline_id }))
}

#[derive(Debug, Deserialize)]
pub struct CreatePipelineRequest {
    name: String,
    description: Option<String>,
    project_id: ProjectId,
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
    db: Extension<Database>,
    Path(pipeline_id): Path<String>,
) -> Result<Json<RunPipelineResponse>, ServiceError> {
    let pipeline_id = PipelineId::try_from(pipeline_id)?;
    let pipeline = db.get_pipeline(&pipeline_id).await?;

    let job = Job::create(pipeline.pipeline_id);
    let job_id = job.job_id.clone();
    db.create_job(job).await?;

    Ok(Json(RunPipelineResponse { job_id }))
}

#[derive(Debug, Serialize)]
pub struct RunPipelineResponse {
    job_id: JobId,
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn delete_pipeline(
    claims: Claims,
    db: Extension<Database>,
    Path(pipeline_id): Path<String>,
) -> Result<(), ServiceError> {
    let pipeline_id = PipelineId::try_from(pipeline_id)?;
    db.delete_pipeline(&pipeline_id).await?;

    Ok(())
}
