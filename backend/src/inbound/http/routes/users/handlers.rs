use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    domain::user::{models::UserId, service::UserService},
    inbound::http::{errors::ApiError, routes::users::models::GetUserHttpResponse, AppState},
};

use crate::domain::auth::models::Claims;

#[axum::debug_handler(state = AppState)]
pub async fn get_current_user(
    claims: Claims,
    State(user_service): State<Arc<dyn UserService>>,
) -> Result<Json<GetUserHttpResponse>, ApiError> {
    let user = user_service
        .get_user_by_id(&UserId::from(claims.sub))
        .await?;
    Ok(Json(user.into()))
}

#[axum::debug_handler(state = AppState)]
pub async fn get_user(
    claims: Claims,
    State(user_service): State<Arc<dyn UserService>>,
    Path(user_id): Path<UserId>,
) -> Result<Json<GetUserHttpResponse>, ApiError> {
    if claims.sub != user_id {
        return Err(ApiError::Forbidden);
    }
    let user = user_service.get_user_by_id(&user_id).await?;
    Ok(Json(user.into()))
}

#[allow(dead_code)]
#[axum::debug_handler(state = AppState)]
pub async fn delete_user(
    claims: Claims,
    State(user_service): State<Arc<dyn UserService>>,
    Path(user_id): Path<UserId>,
) -> Result<(), ApiError> {
    user_service.delete_user(&claims.sub, &user_id).await?;
    Ok(())
}
