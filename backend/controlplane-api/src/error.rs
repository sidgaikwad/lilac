use std::error::Error;

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde_json::json;

use crate::{auth::error::AuthError, database::DatabaseError};

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("{entity_type} not found")]
    EntityNotFound {
        entity_id: String,
        entity_type: String,
    },

    #[error("unauthorized: {reason}")]
    Unauthorized { reason: String },

    #[error("{0}")]
    BadRequest(String),

    #[error("{entity_type} already exists")]
    EntityAlreadyExists {
        entity_type: String,
        entity_id: String,
    },

    #[error("failed to read {source_type} data source: \"{source_id}\"")]
    InvalidDataSource {
        source_type: String,
        source_id: String,
    },

    #[error("{0}")]
    InternalError(String),

    #[error("{0}")]
    Unhandled(Box<dyn Error>),
}

impl ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::EntityNotFound { .. } => StatusCode::NOT_FOUND,
            ServiceError::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            ServiceError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ServiceError::EntityAlreadyExists { .. } => StatusCode::CONFLICT,
            ServiceError::InvalidDataSource { .. } => StatusCode::BAD_REQUEST,
            ServiceError::Unhandled(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_message(&self) -> String {
        match self {
            ServiceError::Unhandled(err) => {
                tracing::error!(error = ?err, "unexpected error");
                String::from("something went wrong")
            }
            ServiceError::InternalError(err) => {
                tracing::error!(error = ?err, "unknown error");
                String::from("something went wrong")
            }
            _ => self.to_string(),
        }
    }
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.error_message(),
        }));
        (self.status_code(), body).into_response()
    }
}

impl From<DatabaseError> for ServiceError {
    fn from(error: DatabaseError) -> Self {
        match error {
            DatabaseError::NotFound {
                entity_type,
                entity_id,
            } => ServiceError::EntityNotFound {
                entity_id,
                entity_type,
            },
            DatabaseError::Conflict {
                entity_type,
                entity_id,
            } => ServiceError::EntityAlreadyExists {
                entity_id,
                entity_type,
            },
            DatabaseError::SqlxError(error) => ServiceError::Unhandled(Box::new(error)),
            DatabaseError::SerdeError(error) => ServiceError::Unhandled(Box::new(error)),
        }
    }
}

impl From<AuthError> for ServiceError {
    fn from(value: AuthError) -> Self {
        match value {
            AuthError::InvalidToken => ServiceError::Unauthorized {
                reason: String::from("invalid token"),
            },
            AuthError::WrongCredentials => ServiceError::Unauthorized {
                reason: String::from("invalid credentials"),
            },
            AuthError::TokenCreation => {
                ServiceError::InternalError(String::from("token creation failed"))
            }
            AuthError::MissingCredentials => ServiceError::Unauthorized {
                reason: String::from("missing credentials"),
            },
            AuthError::InvalidInput(e) => ServiceError::BadRequest(e),
        }
    }
}
