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
    #[error("Provider not found")]
    ProviderNotFound,
    #[error("Missing OIDC cookie")]
    MissingOidcCookie,
    #[error("Invalid OIDC cookie")]
    InvalidOidcCookie,
    #[error("Invalid CSRF token")]
    InvalidCsrfToken,
    #[error("Code exchange failed")]
    CodeExchangeFailed,
    #[error("Missing ID token")]
    MissingIdToken,
    #[error("Claims verification failed")]
    ClaimsVerificationFailed,
    #[error("Invalid access token")]
    InvalidAccessToken,
    #[error("Failed to get user info")]
    UserInfoFailed,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = match self {
            AuthError::InvalidToken => StatusCode::UNAUTHORIZED,
            AuthError::WrongCredentials => StatusCode::UNAUTHORIZED,
            AuthError::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::MissingCredentials => StatusCode::UNAUTHORIZED,
            AuthError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            AuthError::ProviderNotFound => StatusCode::NOT_FOUND,
            AuthError::MissingOidcCookie => StatusCode::BAD_REQUEST,
            AuthError::InvalidOidcCookie => StatusCode::BAD_REQUEST,
            AuthError::InvalidCsrfToken => StatusCode::BAD_REQUEST,
            AuthError::CodeExchangeFailed => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::MissingIdToken => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::ClaimsVerificationFailed => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::InvalidAccessToken => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::UserInfoFailed => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let error_message = self.to_string();
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
