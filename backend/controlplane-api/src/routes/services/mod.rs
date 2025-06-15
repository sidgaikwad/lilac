use axum::{
    routing::{get, post},
    Router,
};
mod services;
use crate::AppState;
use services::*;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/organizations/{organization_id}/services",
            post(start_service).get(list_services),
        )
        .route("/services/{service_id}", get(get_service).delete(delete_service))
}
