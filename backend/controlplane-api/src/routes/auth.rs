use axum::{Extension, Json};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Header};
use password_auth::verify_password;
use secrecy::ExposeSecret;
use serde::Deserialize;
use sqlx::PgPool;

use crate::{auth::{claims::Claims, error::AuthError, keys::KEYS}, database, model::auth::AuthBody};

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    email: String,
    password: String,
}

pub async fn authorize(Extension(db): Extension<PgPool>, Json(request): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    if request.email.is_empty() || request.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let user = database::get_user_by_email(&db, &request.email).await.map_err(|_| AuthError::WrongCredentials)?;
    if verify_password(request.password, &user.password_hash.expose_secret()).is_err() {
        return Err(AuthError::WrongCredentials);
    }


    let exp = (Utc::now() + Duration::hours(6)).timestamp() as usize;
    let claims = Claims {
        email: request.email,
        exp,
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding).map_err(
        |_| AuthError::TokenCreation
    )?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}

