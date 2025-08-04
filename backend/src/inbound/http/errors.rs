use crate::domain::{
    auth::service::AuthServiceError, cluster::service::ClusterServiceError,
    queue::service::QueueServiceError, training_job::service::TrainingJobServiceError,
    user::service::UserServiceError,
};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    InternalServerError(String),
    Conflict(String),
    BadRequest(String),
    UnprocessableEntity(String),
    NotFound(String),
    Unauthorized(String),
    Forbidden,
}

impl From<QueueServiceError> for ApiError {
    fn from(err: QueueServiceError) -> Self {
        match err {
            QueueServiceError::InvalidPermissions => Self::Forbidden,
            QueueServiceError::QueueExists { .. } => Self::Conflict("Queue already exists".into()),
            QueueServiceError::QueueNotFound(_) => Self::NotFound("Queue not found".to_string()),
            QueueServiceError::Unknown(e) => {
                tracing::error!(error = ?e, backtrace = %e.backtrace(), "unknown error occurred");
                Self::InternalServerError("Something went wrong".to_string())
            }
        }
    }
}

impl From<ClusterServiceError> for ApiError {
    fn from(err: ClusterServiceError) -> Self {
        match err {
            ClusterServiceError::InvalidPermissions => Self::Forbidden,
            ClusterServiceError::ClusterExists { .. } => {
                Self::Conflict("Cluster already exists".into())
            }
            ClusterServiceError::ClusterNotFound(_) => {
                Self::NotFound("Cluster not found".to_string())
            }
            ClusterServiceError::Unknown(e) => {
                tracing::error!(error = ?e, backtrace = %e.backtrace(), "unknown error occurred");
                Self::InternalServerError("Something went wrong".to_string())
            }
        }
    }
}

impl From<UserServiceError> for ApiError {
    fn from(err: UserServiceError) -> Self {
        match err {
            UserServiceError::InvalidPermissions => Self::Forbidden,
            UserServiceError::UserExists { .. } => Self::Conflict("User already exists".into()),
            UserServiceError::UserNotFound(_) => Self::NotFound("User not found".to_string()),
            UserServiceError::ApiKeyNotFound => Self::NotFound("API key not found".to_string()),
            UserServiceError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
                Self::InternalServerError("Something went wrong".to_string())
            }
        }
    }
}

impl From<AuthServiceError> for ApiError {
    fn from(err: AuthServiceError) -> Self {
        match err {
            AuthServiceError::InvalidCredentials => {
                Self::Unauthorized("Invalid credentials".to_string())
            }
            AuthServiceError::UserNotFound => Self::Unauthorized("Invalid credentials".to_string()),
            _ => Self::InternalServerError("Something went wrong".to_string()),
        }
    }
}

impl From<TrainingJobServiceError> for ApiError {
    fn from(err: TrainingJobServiceError) -> Self {
        match err {
            TrainingJobServiceError::TrainingJobExists { .. } => {
                Self::Conflict("Cluster already exists".into())
            }
            TrainingJobServiceError::TrainingJobNotFound(_) => {
                Self::NotFound("Cluster not found".to_string())
            }
            TrainingJobServiceError::InvalidDefinition(e) => {
                Self::BadRequest(format!("Invalid job definition: {e}"))
            }
            TrainingJobServiceError::Unknown(e) => {
                tracing::error!(error = ?e, backtrace = %e.backtrace(), "unknown error occurred");
                Self::InternalServerError("Something went wrong".to_string())
            }
        }
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        tracing::error!(error = ?err, "Detailed error: {:?}", err);
        Self::InternalServerError("Something went wrong".to_string())
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        tracing::error!(error = ?err, "unknown error occurred");
        Self::InternalServerError("Something went wrong".to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::UnprocessableEntity(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}
