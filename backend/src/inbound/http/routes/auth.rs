use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::{
    domain::auth::models::Token,
    inbound::http::{responses::ApiError, AppState},
};
use tower_sessions::Session;

use super::user::create_user;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(login_with_email))
        .route("/auth/signup", post(create_user))
        .route("/logout", get(logout))
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    email: String,
    password: String,
}

async fn login_with_email(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Token>, ApiError> {
    let token = app_state
        .auth_service
        .login_with_email(&payload.email, &payload.password)
        .await?;
    Ok(Json(token))
}

async fn logout(session: Session) -> Result<impl IntoResponse, ApiError> {
    session.clear().await;
    Ok(Redirect::to("/"))
}
