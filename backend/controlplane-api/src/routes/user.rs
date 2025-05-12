use axum::{extract::{Path, State}, Json};
use common::{
    database::Database,
    model::user::{User, UserId},
    ServiceError,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use validator::Validate;

use crate::auth::claims::Claims;

#[instrument(level = "info", skip(db), ret, err)]
pub async fn create_user(
    State(db): State<Database>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, ServiceError> {
    match request.validate() {
        Ok(_) => (),
        Err(e) => return Err(ServiceError::SchemaValidationError(e.to_string())),
    }
    let user = User::create(request.email, request.password.into());

    let user_id = db.create_user(user).await?;

    Ok(Json(CreateUserResponse { id: user_id }))
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email(message = "Invalid email format"))]
    #[validate(length(min = 1, message = "Email cannot be empty"))]
    email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    password: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    id: UserId,
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn get_current_user(
    State(db): State<Database>,
    claims: Claims,
) -> Result<Json<GetUserResponse>, ServiceError> {
    let user = db.get_user(&claims.sub).await?;

    Ok(Json(GetUserResponse {
        id: user.user_id,
        email: user.email,
    }))
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn get_user(
    claims: Claims,
    State(db): State<Database>,
    Path(user_id): Path<String>,
) -> Result<Json<GetUserResponse>, ServiceError> {
    let user_id = UserId::try_from(user_id)?;

    if claims.sub != user_id {
        return Err(ServiceError::Unauthorized);
    }

    let user = db.get_user(&user_id).await?;

    Ok(Json(GetUserResponse {
        id: user.user_id,
        email: user.email,
    }))
}

#[derive(Debug, Serialize)]
pub struct GetUserResponse {
    id: UserId,
    email: String,
}
