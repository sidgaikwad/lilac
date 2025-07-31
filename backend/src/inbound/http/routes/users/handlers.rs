use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    domain::user::{
        models::{ApiKeyId, UserId},
        service::UserService,
    },
    inbound::http::{
        errors::ApiError,
        routes::users::models::{ApiKeyResponse, CreateApiKeyResponse, GetUserHttpResponse},
        AppState,
    },
};

use crate::domain::auth::models::Claims;

#[axum::debug_handler(state = AppState)]
pub async fn get_current_user(
    claims: Claims,
    State(user_service): State<Arc<dyn UserService>>,
) -> Result<Json<GetUserHttpResponse>, ApiError> {
    let user = user_service
        .get_user_by_id(&claims.sub)
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

#[axum::debug_handler(state = AppState)]
pub async fn create_api_key(
    claims: Claims,
    State(user_service): State<Arc<dyn UserService>>,
) -> Result<Json<CreateApiKeyResponse>, ApiError> {
    let new_api_key = user_service.create_api_key(&claims.sub).await?;
    Ok(Json(new_api_key.into()))
}

#[axum::debug_handler(state = AppState)]
pub async fn list_api_keys(
    claims: Claims,
    State(user_service): State<Arc<dyn UserService>>,
) -> Result<Json<Vec<ApiKeyResponse>>, ApiError> {
    let api_keys = user_service.list_api_keys(&claims.sub).await?;
    let response = api_keys.into_iter().map(ApiKeyResponse::from).collect();
    Ok(Json(response))
}

#[axum::debug_handler(state = AppState)]
pub async fn delete_api_key(
    claims: Claims,
    State(user_service): State<Arc<dyn UserService>>,
    Path(key_id): Path<ApiKeyId>,
) -> Result<(), ApiError> {
    user_service.delete_api_key(&claims.sub, &key_id).await?;
    Ok(())
}
