use crate::inbound::http::AppState;
use axum::{
    routing::{get, post},
    Router,
};

mod handlers;
use handlers::*;
mod models;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(login_with_email))
        .route("/auth/signup", post(sign_up))
        .route("/auth/logout", get(logout))
}
