use std::sync::Arc;

use super::models::{
    CreateTrainingJobRequest, CreateTrainingJobResponse, PostLogsRequest,
    UpdateTrainingJobStatusRequest,
};
use crate::domain::training_job::models::GetTrainingJobsFilters;
use crate::domain::training_job::service::TrainingJobService;
use crate::inbound::http::routes::training_jobs::models::HttpTrainingJob;
use crate::{
    domain::{auth::models::Claims, training_job::models::JobId},
    inbound::http::{
        errors::ApiError, routes::training_jobs::models::ListTrainingJobsHttpResponse, AppState,
    },
};
use axum::extract::Path;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use secrecy::SecretString;

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

#[axum::debug_handler(state = AppState)]
pub async fn get_training_job(
    _claims: Claims,
    State(training_job_service): State<Arc<dyn TrainingJobService>>,
    Path(job_id): Path<JobId>,
) -> Result<Json<HttpTrainingJob>, ApiError> {
    let training_job = training_job_service.get_training_job_by_id(&job_id).await?;

    Ok(Json(training_job.into()))
}

#[axum::debug_handler]
pub async fn list_training_jobs(
    _claims: Claims,
    State(state): State<AppState>,
    Query(params): Query<GetTrainingJobsFilters>,
) -> Result<Json<ListTrainingJobsHttpResponse>, ApiError> {
    let training_jobs = state.training_job_service.get_training_jobs(params).await?;

    Ok(Json(training_jobs.into()))
}

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
