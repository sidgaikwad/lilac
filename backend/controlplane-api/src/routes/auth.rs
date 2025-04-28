use axum::{Extension, Json};
use common::{database::Database, model::auth::AuthBody};
use jsonwebtoken::{encode, Header};
use password_auth::verify_password;
use secrecy::ExposeSecret;
use serde::Deserialize;

use crate::auth::{claims::Claims, error::AuthError, keys::KEYS};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthPayload {
    email: String,
    password: String,
}

pub async fn authorize(
    Extension(db): Extension<Database>,
    Json(request): Json<AuthPayload>,
) -> Result<Json<AuthBody>, AuthError> {
    if request.email.is_empty() || request.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let user = db
        .get_user_by_email(&request.email)
        .await
        .map_err(|_| AuthError::WrongCredentials)?;
    if verify_password(request.password, &user.password_hash.expose_secret()).is_err() {
        return Err(AuthError::WrongCredentials);
    }

    let claims = Claims::create(user.user_id);
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}
