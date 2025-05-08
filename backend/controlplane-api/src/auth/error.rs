use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

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
    fn into_response(self) -> Response {
        let status = match self {
            AuthError::InvalidToken => StatusCode::UNAUTHORIZED,
            AuthError::WrongCredentials => StatusCode::UNAUTHORIZED,
            AuthError::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::MissingCredentials => StatusCode::UNAUTHORIZED,
            AuthError::InvalidInput(_) => StatusCode::BAD_REQUEST,
        };
        let error_message = self.to_string();
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
