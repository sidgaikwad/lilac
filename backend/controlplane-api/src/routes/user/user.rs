use crate::{
    auth::{claims::Claims, error::AuthError},
    database::Database,
    model::user::{User, UserId},
    ServiceError,
};
use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use validator::Validate;

#[instrument(level = "info", skip(db), ret, err)]
pub async fn create_user(
    State(db): State<Database>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, ServiceError> {
    request
        .validate()
        .map_err(|e| AuthError::InvalidInput(e.to_string()))?;

    if db.get_user_by_email(&request.email).await.is_ok() {
        return Err(ServiceError::EntityAlreadyExists {
            entity_type: "user".into(),
            entity_id: request.email.clone(),
        });
    }

    let user = User::create_password_user(request.email, request.password.into());

    let user_id = db.create_user(user).await?;

    Ok(Json(CreateUserResponse { user_id }))
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
    #[serde(rename = "userId")]
    user_id: UserId,
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn get_current_user(
    State(db): State<Database>,
    claims: Claims,
) -> Result<Json<GetUserResponse>, ServiceError> {
    let user = db.get_user(&claims.sub).await?;

    Ok(Json(GetUserResponse {
        user_id: user.user_id,
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
        return Err(ServiceError::Unauthorized {
            reason: "not allowed to read this user".into(),
        });
    }

    let user = db.get_user(&user_id).await?;

    Ok(Json(GetUserResponse {
        user_id: user.user_id,
        email: user.email,
    }))
}

#[derive(Debug, Serialize)]
pub struct GetUserResponse {
    user_id: UserId,
    email: String,
}
#[instrument(level = "info", skip(db), ret, err)]
pub async fn delete_user_handler(
    claims: Claims,
    State(db): State<Database>,
    Path(user_id_str): Path<String>,
) -> Result<(), ServiceError> {
    let user_id_to_delete = UserId::try_from(user_id_str)?;

    if user_id_to_delete != claims.sub {
        return Err(ServiceError::Unauthorized {
            reason: "not allowed to delete this user".into(),
        });
    }

    db.delete_user(&user_id_to_delete).await?;

    Ok(())
}
