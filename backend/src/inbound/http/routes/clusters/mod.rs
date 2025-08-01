use axum::{
    routing::{delete, get, post},
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
        .route("/clusters/{cluster_id}/info", get(get_cluster_info))
        .route("/clusters/{cluster_id}/nodes", get(list_cluster_nodes))
        .route(
            "/clusters/{cluster_id}/api-keys",
            post(create_api_key_for_cluster).get(list_api_keys),
        )
        .route(
            "/clusters/{cluster_id}/api-keys/{key_id}",
            delete(delete_cluster_api_key),
        )
        .route("/clusters/{cluster_id}/jobs", get(list_cluster_jobs))
        .route("/node/{node_id}/status", post(cluster_node_heartbeat))
}
