use axum::{routing::post, Router};

mod auth;

use crate::AppState;
use auth::authorize;
use serde::Serialize;

pub fn router() -> Router<AppState> {
    Router::new().route("/auth/login", post(authorize))
}
