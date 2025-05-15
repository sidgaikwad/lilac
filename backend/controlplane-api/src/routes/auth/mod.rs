use axum::{routing::post, Router};
mod auth;
use auth::authorize;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/auth/login", post(authorize))
}