use axum::{extract::Path, Extension, Json};
use chrono::{DateTime, Utc};
use common::{
    database::Database,
    model::user::{User, UserId},
    ServiceError,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::auth::claims::Claims;

#[instrument(level = "info", skip(db), ret, err)]
pub async fn create_user(
    db: Extension<Database>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, ServiceError> {
    let user = User::create(request.email, request.password.into());

    let user_id = db.create_user(user).await?;

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
    claims: Claims,
    db: Extension<Database>,
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
        created_at: user.created_at,
    }))
}

#[derive(Debug, Serialize)]
pub struct GetUserResponse {
    id: UserId,
    email: String,
    created_at: DateTime<Utc>,
}
