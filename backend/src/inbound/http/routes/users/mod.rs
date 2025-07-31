use axum::{
    routing::{delete, get},
    Router,
};

use crate::inbound::http::AppState;

mod handlers;
use handlers::*;
mod models;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/account/details", get(get_current_user))
        .route("/users/{id}", get(get_user))
        .route("/account/api-keys", get(list_api_keys).post(create_api_key))
        .route("/account/api-keys/{key_id}", delete(delete_api_key))
}
