use uuid::Uuid;
use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{
    domain::training_job::models::TrainingJobStatus,
    inbound::http::AppState,
};

use super::models::{CreateTrainingJobRequest, CreateTrainingJobResponse, UpdateTrainingJobStatusRequest, PostLogsRequest};

pub async fn create_training_job(
    State(state): State<AppState>,
    Json(request): Json<CreateTrainingJobRequest>,
) -> impl IntoResponse {
    let training_job = state
        .training_job_service
        .create(request.name, request.definition, request.cluster_id)
        .await
        .unwrap();

    (StatusCode::CREATED, Json(CreateTrainingJobResponse::from(training_job)))
}

use crate::domain::training_job::models::GetTrainingJobsFilters;

#[axum::debug_handler]
pub async fn get_training_jobs(
    State(state): State<AppState>,
    Query(params): Query<GetTrainingJobsFilters>,
) -> impl IntoResponse {
    let training_jobs = state
        .training_job_service
        .get_training_jobs(params)
        .await
        .unwrap();

    (StatusCode::OK, Json(training_jobs))
}
use axum::extract::Path;

pub async fn schedule_training_job(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> impl IntoResponse {
    // TODO: Implement the logic to schedule the job
    (StatusCode::OK, Json(()))
}

pub async fn update_training_job_status(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
    Json(request): Json<UpdateTrainingJobStatusRequest>,
) -> impl IntoResponse {
    state
        .training_job_service
        .update_status(job_id, request.status)
        .await
        .unwrap();

    (StatusCode::OK, Json(()))
}

pub async fn post_logs(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
    Json(request): Json<PostLogsRequest>,
) -> impl IntoResponse {
    // TODO: Implement log ingestion
    (StatusCode::OK, Json(()))
}