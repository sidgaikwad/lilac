use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde_json::json;

pub mod auth;
pub mod database;
pub mod model;
pub mod routes;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("parse error: {0}")]
    ParseError(String),

    #[error("database error: {0}")]
    ServiceError(#[from] sqlx::Error),
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ServiceError::ParseError(_) => (StatusCode::BAD_REQUEST, "bad request"),
            ServiceError::ServiceError(_) => (StatusCode::NOT_FOUND, "not found"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
