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
        .route(
            "/credentials",
            post(create_credential).get(list_credentials),
        )
        .route(
            "/credentials/{credential_id}",
            get(get_credential).delete(delete_credential),
        )
}
