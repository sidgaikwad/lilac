use axum::{routing::{get, post}, Router};
mod datasets;
use crate::AppState;
use datasets::{
    connect_data_source, delete_dataset_handler, get_dataset, list_datasets_handler,
    test_data_source_connection,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/projects/{projectId}/datasets",
            get(list_datasets_handler).post(connect_data_source),
        )
        .route(
            "/datasets/{datasetId}",
            get(get_dataset).delete(delete_dataset_handler),
        )
        .route(
            "/projects/{projectId}/datasets/test",
            post(test_data_source_connection),
        )
}
