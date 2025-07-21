use crate::domain::{
    auth::service::AuthServiceError, cluster::service::ClusterServiceError,
    credentials::service::CredentialServiceError, dataset::service::DatasetServiceError,
    project::service::ProjectServiceError, user::service::UserServiceError,
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

impl From<CredentialServiceError> for ApiError {
    fn from(err: CredentialServiceError) -> Self {
        match err {
            CredentialServiceError::InvalidPermissions => Self::Forbidden,
            CredentialServiceError::CredentialExists { .. } => {
                Self::Conflict("Cluster already exists".into())
            }
            CredentialServiceError::CredentialNotFound(_) => {
                Self::NotFound(format!("Cluster not found"))
            }
            CredentialServiceError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
                Self::InternalServerError("Something went wrong".to_string())
            }
        }
    }
}

impl From<ClusterServiceError> for ApiError {
    fn from(err: ClusterServiceError) -> Self {
        match err {
            ClusterServiceError::IncorrectCredentialsType => {
                Self::BadRequest("Incorrect credentials type".into())
            }
            ClusterServiceError::InvalidPermissions => Self::Forbidden,
            ClusterServiceError::ClusterExists { .. } => {
                Self::Conflict("Cluster already exists".into())
            }
            ClusterServiceError::ClusterNotFound(_) => Self::NotFound(format!("Cluster not found")),
            ClusterServiceError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
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
            UserServiceError::UserNotFound(_) => Self::NotFound(format!("User not found")),
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
            ProjectServiceError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
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
            DatasetServiceError::Unknown(cause) => {
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
            WorkspaceServiceError::KubeClientFactory(err) => {
                tracing::error!("Kube client factory error: {:?}", err);
                Self::InternalServerError("Something went wrong".to_string())
            }
            WorkspaceServiceError::Unexpected => {
                Self::InternalServerError("Something went wrong".to_string())
            }
        }
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        tracing::error!("{:?}\n{}", err, err.backtrace());
        Self::InternalServerError("Something went wrong".to_string())
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        tracing::error!("{:?}", err);
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
