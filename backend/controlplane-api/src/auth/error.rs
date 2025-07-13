use axum::response::IntoResponse;

use crate::ServiceError;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Token creation error")]
    TokenCreation,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        ServiceError::from(self).into_response()
    }
}
