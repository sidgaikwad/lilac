use crate::domain::{
    auth::service::AuthServiceError, cluster::service::ClusterServiceError,
    credentials::service::CredentialServiceError, dataset::service::DatasetServiceError,
    project::service::ProjectServiceError, queue::service::QueueServiceError,
    training_job::service::TrainingJobServiceError, user::service::UserServiceError,
    workspace::service::WorkspaceServiceError,
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

impl From<CredentialServiceError> for ApiError {
    fn from(err: CredentialServiceError) -> Self {
        match err {
            CredentialServiceError::InvalidPermissions => Self::Forbidden,
            CredentialServiceError::CredentialExists { .. } => {
                Self::Conflict("Cluster already exists".into())
            }
            CredentialServiceError::CredentialNotFound(_) => {
                Self::NotFound("Cluster not found".to_string())
            }
            CredentialServiceError::Unknown(e) => {
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

impl From<ProjectServiceError> for ApiError {
    fn from(err: ProjectServiceError) -> Self {
        match err {
            ProjectServiceError::InvalidPermissions => Self::Forbidden,
            ProjectServiceError::ProjectExists { .. } => {
                Self::Conflict("Project already exists".into())
            }
            ProjectServiceError::ProjectNotFound(_) => Self::NotFound("Project not found".into()),
            ProjectServiceError::Unknown(e) => {
                tracing::error!(error = ?e, backtrace = %e.backtrace(), "unknown error occurred");
                Self::InternalServerError("Something went wrong".to_string())
            }
        }
    }
}

impl From<DatasetServiceError> for ApiError {
    fn from(err: DatasetServiceError) -> Self {
        match err {
            DatasetServiceError::DatasetExists { .. } => {
                Self::Conflict("Dataset already exists".into())
            }
            DatasetServiceError::DatasetNotFound(_) => Self::NotFound("Dataset not found".into()),
            DatasetServiceError::ConnectionError(_) => {
                Self::BadRequest("Failed to connect to data source".into())
            }
            DatasetServiceError::InvalidPermissions => Self::Forbidden,
            DatasetServiceError::Unknown(e) => {
                tracing::error!(error = ?e, backtrace = %e.backtrace(), "unknown error occurred");
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

impl From<WorkspaceServiceError> for ApiError {
    fn from(err: WorkspaceServiceError) -> Self {
        match err {
            WorkspaceServiceError::Repository(err) => {
                tracing::error!("Repository error: {:?}", err);
                Self::InternalServerError("Something went wrong".to_string())
            }
            WorkspaceServiceError::Provisioner(err) => {
                tracing::error!("Provisioner error: {:?}", err);
                Self::InternalServerError("Something went wrong".to_string())
            }
            WorkspaceServiceError::ClusterRepository(err) => {
                tracing::error!("Cluster repository error: {:?}", err);
                Self::InternalServerError("Something went wrong".to_string())
            }
            WorkspaceServiceError::CredentialRepository(err) => {
                tracing::error!("Credential repository error: {:?}", err);
                Self::InternalServerError("Something went wrong".to_string())
            }
            WorkspaceServiceError::Unknown(e) => {
                tracing::error!(error = ?e, backtrace = %e.backtrace(), "unknown error occurred");
                Self::InternalServerError("Something went wrong".to_string())
            }
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
