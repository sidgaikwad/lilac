use axum::{
    routing::{get, post},
    Router,
};

use crate::inbound::http::AppState;

mod handlers;
use handlers::*;
mod models;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/datasets", post(create_dataset).get(list_datasets))
        .route("/datasets/test", post(test_data_source_connection))
        .route(
            "/datasets/{dataset_id}",
            get(get_dataset).delete(delete_dataset),
        )
}
