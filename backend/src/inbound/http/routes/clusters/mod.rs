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
        .route(
            "/clusters/{cluster_id}",
            get(get_cluster).delete(delete_cluster),
        )
        .route("/clusters/{cluster_id}/nodes", get(list_cluster_nodes))
        .route(
            "/clusters/{cluster_id}/api_keys",
            post(create_api_key_for_cluster).get(list_api_keys),
        )
        .route("/node/{node_id}/status", post(cluster_node_heartbeat))
}
