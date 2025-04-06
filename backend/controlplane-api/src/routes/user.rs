use axum::{extract::Path, Extension, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::instrument;

use crate::{
    auth::claims::Claims,
    database,
    model::user::{User, UserId},
    ServiceError,
};

#[instrument(level = "info", skip(db), ret, err)]
pub async fn create_user(
    db: Extension<PgPool>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, ServiceError> {
    let user = User::create(request.email, request.password.into());

    let user_id = database::create_user(&db, user).await?;

    Ok(Json(CreateUserResponse { id: user_id }))
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    id: UserId,
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn get_user(
    _claims: Claims,
    db: Extension<PgPool>,
    Path(user_id): Path<String>,
) -> Result<Json<GetUserResponse>, ServiceError> {
    let user_id = UserId::try_from(user_id)?;

    let user = database::get_user(&db, &user_id).await?;

    Ok(Json(GetUserResponse {
        id: user.user_id,
        email: user.email,
        created_at: user.created_at,
    }))
}

#[derive(Debug, Serialize)]
pub struct GetUserResponse {
    id: UserId,
    email: String,
    created_at: DateTime<Utc>,
}
