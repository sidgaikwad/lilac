use axum::{routing::{get, post, delete}, Router};
mod datasets;
use datasets::{create_dataset, get_dataset, list_datasets_handler, delete_dataset_handler};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/datasets", get(list_datasets_handler).post(create_dataset))
        .route("/datasets/{datasetId}", get(get_dataset).delete(delete_dataset_handler))
}