use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    Json,
};

use crate::{
    domain::auth::models::Token,
    inbound::http::{
        errors::ApiError,
        routes::auth::models::{LoginHttpRequest, SignUpHttpRequest, SignUpHttpResponse},
        AppState,
    },
};
use tower_sessions::Session;

pub async fn login_with_email(
    State(app_state): State<AppState>,
    Json(req): Json<LoginHttpRequest>,
) -> Result<Json<Token>, ApiError> {
    let token = app_state
        .auth_service
        .login_with_email(&req.email, &req.password)
        .await?;
    Ok(Json(token))
}

pub async fn logout(session: Session) -> Result<impl IntoResponse, ApiError> {
    session.clear().await;
    Ok(Redirect::to("/"))
}

pub async fn sign_up(
    State(app_state): State<AppState>,
    Json(req): Json<SignUpHttpRequest>,
) -> Result<Json<SignUpHttpResponse>, ApiError> {
    let user = app_state.user_service.create_user(&req.into()).await?;
    Ok(Json(SignUpHttpResponse { user_id: user.id }))
}
