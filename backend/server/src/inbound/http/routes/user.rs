use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{
    domain::{
        user::{
            models::{CreateUserRequest, UserId},
            service::UserService,
        },
    },
    inbound::http::{responses::{ApiError, UserResponse}, AppState},
};

use crate::domain::auth::models::Claims;


use axum::routing::{get};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/account/details", get(get_current_user))
        .route("/users/{id}", get(get_user))
    // .route("/users/:id", delete(delete_user))
}

#[axum::debug_handler(state = AppState)]
pub async fn create_user(
    State(user_service): State<Arc<dyn UserService>>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = user_service.create_user(&req).await?;
    Ok(Json(user.into()))
}

#[axum::debug_handler(state = AppState)]
async fn get_current_user(
    State(user_service): State<Arc<dyn UserService>>,
    claims: Claims,
) -> Result<Json<UserResponse>, ApiError> {
    let user = user_service.get_user_by_id(&UserId(claims.sub)).await?;
    Ok(Json(user.into()))
}

#[axum::debug_handler(state = AppState)]
pub async fn get_user(
    State(user_service): State<Arc<dyn UserService>>,
    claims: Claims,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    if claims.sub != user_id {
        return Err(ApiError::Forbidden);
    }
    let user = user_service.get_user_by_id(&UserId(user_id)).await?;
    Ok(Json(user.into()))
}

#[axum::debug_handler(state = AppState)]
pub async fn delete_user(
    State(user_service): State<Arc<dyn UserService>>,
    claims: Claims,
    Path(user_id): Path<Uuid>,
) -> Result<(), ApiError> {
    user_service
        .delete_user(&UserId(claims.sub), &UserId(user_id))
        .await?;
    Ok(())
}