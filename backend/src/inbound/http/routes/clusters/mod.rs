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
        .route("/clusters", post(create_cluster).get(list_clusters))
        .route("/clusters/test", post(test_cluster_connection))
        .route(
            "/clusters/{cluster_id}",
            get(get_cluster).delete(delete_cluster),
        )
}
