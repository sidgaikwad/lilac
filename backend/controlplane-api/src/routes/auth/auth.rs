use axum::{extract::State, Json};
use common::{database::Database, model::auth::AuthBody};
use jsonwebtoken::{encode, Header};
use password_auth::verify_password;
use secrecy::ExposeSecret;
use serde::Deserialize;
use validator::Validate;

use crate::auth::{claims::Claims, error::AuthError, keys::KEYS};

#[derive(Debug, Deserialize, Validate)]
pub struct AuthPayload {
    #[validate(email(message = "Invalid email format"))]
    #[validate(length(min = 1, message = "Email cannot be empty"))]
    email: String,
    #[validate(length(min = 1, message = "Password cannot be empty"))]
    password: String,
}

pub async fn authorize(
    State(db): State<Database>,
    Json(request): Json<AuthPayload>,
) -> Result<Json<AuthBody>, AuthError> {
    match request.validate() {
        Ok(_) => (),
        Err(e) => return Err(AuthError::InvalidInput(e.to_string())),
    };

    let user = db
        .get_user_by_email(&request.email)
        .await
        .map_err(|_db_error| AuthError::WrongCredentials)?; // Assuming db error implies wrong creds for simplicity
    if let Some(password_hash) = &user.password_hash {
        if verify_password(request.password, password_hash.expose_secret()).is_err() {
            return Err(AuthError::WrongCredentials);
        }
    } else {
        return Err(AuthError::WrongCredentials);
    }

    let claims = Claims::create(user.user_id);
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_encode_error| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}