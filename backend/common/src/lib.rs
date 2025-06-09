use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub mod aws;
pub mod database;
pub mod model;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("parse error: {0}")]
    ParseError(String),

    #[error("serde error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("{id} not found")]
    NotFound { id: String },

    #[error("unauthorized")]
    Unauthorized,

    #[error("schema validation failed")]
    SchemaError,

    #[error("pipeline execution error: {0}")]
    PipelineExecutionError(String),

    #[error("{0}")]
    BadRequest(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("invalid parameter for {0}: {1}")]
    InvalidParameterValue(String, String),

    #[error("invalid parameter type for {0}: {1}")]
    InvalidParameterType(String, String),

    #[error("missing parameter {0}")]
    MissingParameter(String),

    #[error("pipe execution failed: {0}")]
    PipeError(String),

    #[error("feature not implemented: {0}")]
    NotImplemented(String),

    #[error("schema validation failed: {0}")]
    SchemaValidationError(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("unhandled error")]
    UnhandledError,
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ServiceError::ParseError(_) => (StatusCode::BAD_REQUEST, "bad request".to_string()),
            ServiceError::NotFound { id } => (StatusCode::NOT_FOUND, format!("{id} not found")),
            ServiceError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized".to_string()),
            ServiceError::DatabaseError(sqlx::Error::RowNotFound) => {
                (StatusCode::NOT_FOUND, "not found".to_string())
            }
            ServiceError::DatabaseError(_) | ServiceError::PipelineExecutionError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "something went wrong".to_string(),
            ),
            ServiceError::SchemaError => (
                StatusCode::BAD_REQUEST,
                "schema validation failed".to_string(),
            ),
            ServiceError::Conflict(s) => (StatusCode::CONFLICT, s),
            ServiceError::BadRequest(s) => (StatusCode::BAD_REQUEST, s),
            ServiceError::InvalidParameterValue(_, _) => (
                StatusCode::BAD_REQUEST,
                "invalid parameter value".to_string(),
            ),
            ServiceError::InvalidParameterType(_, _) => (
                StatusCode::BAD_REQUEST,
                "invalid parameter type".to_string(),
            ),
            ServiceError::MissingParameter(_) => {
                (StatusCode::BAD_REQUEST, "missing parameter".to_string())
            }
            // Ensure SchemaValidationError is handled if it has a distinct message requirement
            ServiceError::SchemaValidationError(s) => (StatusCode::BAD_REQUEST, s),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "something went wrong".to_string(),
            ),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
