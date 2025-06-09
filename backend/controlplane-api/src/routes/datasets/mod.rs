use axum::{routing::get, Router};
mod datasets;
use datasets::{create_dataset, delete_dataset_handler, get_dataset, list_dataset_s3_folders, list_datasets_handler};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/projects/{projectId}/datasets", get(list_datasets_handler).post(create_dataset))
        .route("/datasets/{datasetId}", get(get_dataset).delete(delete_dataset_handler))
        .route("/datasets/{datasetId}/s3", get(list_dataset_s3_folders))
}