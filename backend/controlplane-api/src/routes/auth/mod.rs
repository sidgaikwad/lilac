use axum::{routing::{get, post}, Router};

mod auth;
mod oidc;

use crate::AppState;
use auth::authorize;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(authorize))
        .route("/auth/oidc/login/{provider}", get(oidc::login))
        .route("/auth/oidc/callback", get(oidc::callback))
        .route("/auth/oidc/providers", get(oidc::providers))
}