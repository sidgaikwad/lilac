use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub mod database;
pub mod model;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("parse error: {0}")]
    ParseError(String),

    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("{id} not found")]
    NotFound { id: String },

    #[error("unauthorized")]
    Unauthorized,

    #[error("schema validation failed")]
    SchemaError,

    #[error("pipeline execution error")]
    PipelineExecutionError,
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            ServiceError::ParseError(_) => (StatusCode::BAD_REQUEST, "bad request".to_string()),
            ServiceError::NotFound { id } => (StatusCode::NOT_FOUND, format!("{id} not found")),
            ServiceError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized".to_string()),
            ServiceError::DatabaseError(sqlx::Error::RowNotFound) => {
                (StatusCode::NOT_FOUND, "not found".to_string())
            }
            ServiceError::DatabaseError(_) | ServiceError::PipelineExecutionError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "something went wrong".to_string(),
            ),
            ServiceError::SchemaError => (
                StatusCode::BAD_REQUEST,
                "schema validation failed".to_string(),
            ),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
