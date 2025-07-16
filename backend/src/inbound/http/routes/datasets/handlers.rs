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
            models::{DatasetId, DatasetSource},
            service::DatasetService,
        },
    },
    inbound::http::{
        errors::ApiError,
        routes::datasets::models::{
            CreateDatasetHttpRequest, CreateDatasetHttpResponse, GetDatasetHttpResponse,
            ListDatasetsHttpResponse, TestConnectionHttpResponse,
        },
    },
};

use crate::inbound::http::AppState;

#[axum::debug_handler(state = AppState)]
pub async fn create_dataset(
    _claims: Claims,
    State(dataset_service): State<Arc<dyn DatasetService>>,
    Json(req): Json<CreateDatasetHttpRequest>,
) -> Result<Json<CreateDatasetHttpResponse>, ApiError> {
    let dataset = dataset_service.create_dataset(&req.into()).await?;
    Ok(Json(CreateDatasetHttpResponse {
        dataset_id: dataset.id,
    }))
}

#[axum::debug_handler(state = AppState)]
pub async fn get_dataset(
    _claims: Claims,
    State(dataset_service): State<Arc<dyn DatasetService>>,
    Path(dataset_id): Path<DatasetId>,
) -> Result<Json<GetDatasetHttpResponse>, ApiError> {
    let dataset = dataset_service
        .get_dataset_by_id(&dataset_id.into())
        .await?;
    Ok(Json(dataset.into()))
}

#[axum::debug_handler(state = AppState)]
pub async fn list_datasets(
    _claims: Claims,
    State(dataset_service): State<Arc<dyn DatasetService>>,
) -> Result<Json<ListDatasetsHttpResponse>, ApiError> {
    let datasets = dataset_service.list_datasets().await?;
    Ok(Json(datasets.into()))
}

#[axum::debug_handler(state = AppState)]
pub async fn delete_dataset(
    _claims: Claims,
    State(dataset_service): State<Arc<dyn DatasetService>>,
    Path(dataset_id): Path<Uuid>,
) -> Result<(), ApiError> {
    dataset_service.delete_dataset(&dataset_id.into()).await?;
    Ok(())
}

#[axum::debug_handler(state = AppState)]
pub async fn test_data_source_connection(
    State(dataset_service): State<Arc<dyn DatasetService>>,
    Json(source): Json<DatasetSource>,
) -> Result<Json<TestConnectionHttpResponse>, ApiError> {
    dataset_service.test_data_source_connection(&source).await?;
    Ok(Json(TestConnectionHttpResponse { success: true }))
}
