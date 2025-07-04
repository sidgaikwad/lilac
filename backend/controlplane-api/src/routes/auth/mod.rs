use axum::{routing::{get, post}, Router};

mod auth;
mod oidc;
mod oauth2;
mod sso;

use crate::AppState;
use auth::authorize;
use axum::response::IntoResponse;
use axum::extract::State;
use serde::Serialize;

#[derive(Serialize)]
struct Provider {
    name: String,
    r#type: String,
}

async fn providers(State(app_state): State<AppState>) -> impl IntoResponse {
    let mut all_providers = Vec::new();

    for provider in app_state.oidc_configs.keys() {
        all_providers.push(Provider {
            name: provider.clone(),
            r#type: "oidc".to_string(),
        });
    }

    for provider in app_state.oauth2_configs.keys() {
        all_providers.push(Provider {
            name: provider.clone(),
            r#type: "oauth2".to_string(),
        });
    }

    axum::Json(all_providers)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(authorize))
        .route("/auth/oidc/{provider}/login", post(oidc::login))
        .route("/auth/oidc/{provider}/exchange", post(oidc::exchange))
        .route("/auth/oauth2/{provider}/login", post(oauth2::login))
        .route("/auth/oauth2/{provider}/exchange", post(oauth2::exchange))
        .route("/auth/providers", get(providers))
}