use axum::{response::{IntoResponse, Response}, Json, http::StatusCode};
use secrecy::ExposeSecret;
use serde_json::json;
use sqlx::PgPool;

use crate::model::user::{User, UserId};

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("parse error: {0}")]
    ParseError(String),

    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error)
}

impl IntoResponse for DatabaseError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            DatabaseError::ParseError(_) => (StatusCode::BAD_REQUEST, "bad request"),
            DatabaseError::DatabaseError(_) => (StatusCode::NOT_FOUND, "not found"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

pub async fn get_user(db: &PgPool, user_id: &UserId) -> Result<User, DatabaseError>  {
    let id = user_id.inner();
    let user = sqlx::query_as!(
        User,
        // language=PostgreSQL
        r#"
            SELECT * FROM "users" WHERE user_id = $1
        "#,
        id
    )
    .fetch_one(db)
    .await?;
    Ok(user)
}


pub async fn get_user_by_email(db: &PgPool, email: &String) -> Result<User, DatabaseError>  {
    let user = sqlx::query_as!(
        User,
        // language=PostgreSQL
        r#"
            SELECT * FROM "users" WHERE email = $1
        "#,
        email
    )
    .fetch_one(db)
    .await?;
    Ok(user)
}


pub async fn create_user(db: &PgPool, user: User) -> Result<UserId, DatabaseError>  {
    let user_id = sqlx::query!(
        // language=PostgreSQL
        r#"
            INSERT INTO "users" (user_id, email, email_verified, password_hash, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING user_id
        "#,
        user.user_id.inner(),
        &user.email,
        &user.email_verified,
        &user.password_hash.expose_secret(),
        &user.created_at,
    )
    .map(|row| UserId::new(row.user_id))
    .fetch_one(db)
    .await?;
    Ok(user_id)
}