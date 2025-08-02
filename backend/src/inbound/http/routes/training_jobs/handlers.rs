use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    domain::{auth::models::Claims, training_job::models::JobId},
    inbound::http::{errors::ApiError, AppState},
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use secrecy::SecretString;

use super::models::{
    CreateTrainingJobRequest, CreateTrainingJobResponse, PostLogsRequest,
    UpdateTrainingJobStatusRequest,
};

pub async fn create_training_job(
    State(state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(request): Json<CreateTrainingJobRequest>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .user_service
        .authenticate_by_api_key(&SecretString::from(auth.token().to_string()))
        .await?;

    let training_job_with_targets = state.training_job_service.create(request).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateTrainingJobResponse::from(training_job_with_targets)),
    ))
}

use crate::domain::training_job::models::GetTrainingJobsFilters;

#[axum::debug_handler]
pub async fn get_training_jobs(
    _claims: Claims,
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

pub async fn update_training_job_status(
    State(state): State<AppState>,
    Path(job_id): Path<JobId>,
    Json(request): Json<UpdateTrainingJobStatusRequest>,
) -> impl IntoResponse {
    state
        .training_job_service
        .update_status(&job_id, request.status)
        .await
        .unwrap();

    (StatusCode::OK, Json(()))
}

pub async fn post_logs(
    State(_state): State<AppState>,
    Path(_job_id): Path<JobId>,
    Json(_request): Json<PostLogsRequest>,
) -> impl IntoResponse {
    // TODO: Implement log ingestion
    (StatusCode::OK, Json(()))
}

pub async fn cancel_training_job(
    _claims: Claims,
    State(state): State<AppState>,
    Path(job_id): Path<JobId>,
) -> Result<impl IntoResponse, ApiError> {
    state.training_job_service.cancel(&job_id).await?;
    Ok((StatusCode::OK, Json(())))
}
