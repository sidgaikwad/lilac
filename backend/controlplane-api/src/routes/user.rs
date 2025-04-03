use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{auth::claims::Claims, database, model::user::{User, UserId}};

pub async fn create_user(db: Extension<PgPool>, Json(request): Json<CreateUserRequest>) -> impl IntoResponse {
    let user = User::create(request.email, request.password.into());

    let db_res = database::create_user(&db, user).await;

    match db_res {
        Ok(user_id) => {
            (StatusCode::CREATED, Json( CreateUserResponse { id: user_id })).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response(),
    }
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct CreateUserResponse {
    id: UserId,
}

pub async fn get_user(_claims: Claims, db: Extension<PgPool>, Path(user_id): Path<String>) -> impl IntoResponse {
    let user_id = UserId::try_from(user_id);
    match user_id {
        Ok(user_id) => {
            let user = database::get_user(&db, &user_id).await;
            match user {
                Ok(user) => (StatusCode::OK, Json(GetUserResponse {
                    id: user.user_id,
                    email: user.email,
                    created_at: user.created_at,
                })).into_response(),
                Err(_) => (StatusCode::NOT_FOUND, "not found").into_response(),
            }
            
        }
        Err(_) => (StatusCode::BAD_REQUEST, "invalid user id").into_response()
    }

}

#[derive(Serialize)]
struct GetUserResponse {
    id: UserId,
    email: String,
    created_at: DateTime<Utc>
}