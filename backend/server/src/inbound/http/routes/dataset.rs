use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{
    domain::{
        auth::models::Claims,
        dataset::{
            models::{CreateDatasetRequest, Dataset, DatasetId, DatasetSource, DatasetSummary},
            ports::DatasetService,
        },
        project::models::ProjectId,
        user::models::UserId,
    },
    inbound::http::responses::ApiError,
};

use crate::inbound::http::AppState;

use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/projects/{project_id}/datasets",
            post(create_dataset).get(list_datasets),
        )
        .route(
            "/projects/{project_id}/datasets/test",
            post(test_data_source_connection),
        )
        .route(
            "/datasets/{dataset_id}",
            get(get_dataset).delete(delete_dataset),
        )
}

#[axum::debug_handler(state = AppState)]
pub async fn create_dataset(
    State(dataset_service): State<Arc<dyn DatasetService>>,
    claims: Claims,
    Json(req): Json<CreateDatasetRequest>,
) -> Result<Json<Dataset>, ApiError> {
    let mut req = req.clone();
    if req.source.is_none() {
        req.source = Some(DatasetSource::default());
    }

    let dataset = dataset_service
        .create_dataset(&UserId(claims.sub), &req)
        .await?;
    Ok(Json(dataset))
}

#[axum::debug_handler(state = AppState)]
pub async fn get_dataset(
    State(dataset_service): State<Arc<dyn DatasetService>>,
    claims: Claims,
    Path(dataset_id): Path<Uuid>,
) -> Result<Json<Dataset>, ApiError> {
    let dataset = dataset_service
        .get_dataset_by_id(&UserId(claims.sub), &DatasetId(dataset_id))
        .await?;
    Ok(Json(dataset))
}

#[axum::debug_handler(state = AppState)]
pub async fn list_datasets(
    State(dataset_service): State<Arc<dyn DatasetService>>,
    claims: Claims,
    Path(project_id): Path<Uuid>,
) -> Result<Json<ListDatasetsResponse>, ApiError> {
    let datasets = dataset_service
        .list_datasets_by_project_id(&UserId(claims.sub), &ProjectId(project_id))
        .await?;
    let summaries = datasets.into_iter().map(DatasetSummary::from).collect();
    Ok(Json(ListDatasetsResponse {
        datasets: summaries,
    }))
}

#[derive(serde::Serialize)]
pub struct ListDatasetsResponse {
    datasets: Vec<DatasetSummary>,
}

#[axum::debug_handler(state = AppState)]
pub async fn delete_dataset(
    State(dataset_service): State<Arc<dyn DatasetService>>,
    claims: Claims,
    Path(dataset_id): Path<Uuid>,
) -> Result<(), ApiError> {
    dataset_service
        .delete_dataset(&UserId(claims.sub), &DatasetId(dataset_id))
        .await?;
    Ok(())
}

#[axum::debug_handler(state = AppState)]
pub async fn test_data_source_connection(
    State(dataset_service): State<Arc<dyn DatasetService>>,
    Json(source): Json<DatasetSource>,
) -> Result<Json<TestConnectionResponse>, ApiError> {
    dataset_service
        .test_data_source_connection(&source)
        .await?;
    Ok(Json(TestConnectionResponse { success: true }))
}

#[derive(serde::Serialize)]
pub struct TestConnectionResponse {
    pub success: bool,
}