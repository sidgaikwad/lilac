use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;
use serde_json::json;
use crate::domain::{
    dataset::ports::{DatasetRepositoryError, DatasetServiceError},
    user::ports::UserRepositoryError,
    project::ports::ProjectRepositoryError,
};

#[derive(Debug)]
pub struct ApiSuccess<T> {
    status_code: StatusCode,
    data: T,
}

impl<T> ApiSuccess<T> {
    pub fn new(status_code: StatusCode, data: T) -> Self {
        Self { status_code, data }
    }
}

impl<T: Serialize> IntoResponse for ApiSuccess<T> {
    fn into_response(self) -> Response {
        (self.status_code, Json(self.data)).into_response()
    }
}

#[derive(Debug)]
pub enum ApiError {
    InternalServerError(String),
    UnprocessableEntity(String),
    NotFound(String),
    Forbidden,
}

impl From<UserRepositoryError> for ApiError {
    fn from(err: UserRepositoryError) -> Self {
        match err {
            UserRepositoryError::Duplicate(field, value) => {
                Self::UnprocessableEntity(format!("User with {} {} already exists", field, value))
            }
            UserRepositoryError::NotFound(id) => Self::NotFound(format!("User with id {} not found", id)),
            UserRepositoryError::InvalidInput(msg) => Self::UnprocessableEntity(msg),
            UserRepositoryError::Unauthorized => Self::Forbidden,
            UserRepositoryError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
                Self::InternalServerError("Internal server error".to_string())
            }
        }
    }
}

impl From<ProjectRepositoryError> for ApiError {
    fn from(err: ProjectRepositoryError) -> Self {
        match err {
            ProjectRepositoryError::Duplicate(field, value) => {
                Self::UnprocessableEntity(format!("Project with {} {} already exists", field, value))
            }
            ProjectRepositoryError::NotFound(id) => Self::NotFound(format!("Project with id {} not found", id)),
            ProjectRepositoryError::InvalidInput(msg) => Self::UnprocessableEntity(msg),
            ProjectRepositoryError::Unauthorized => Self::Forbidden,
            ProjectRepositoryError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
                Self::InternalServerError("Internal server error".to_string())
            }
        }
    }
}

impl From<DatasetServiceError> for ApiError {
    fn from(err: DatasetServiceError) -> Self {
        match err {
            DatasetServiceError::Repository(repo_err) => match repo_err {
                DatasetRepositoryError::NotFound(id) => {
                    Self::NotFound(format!("Dataset with id {} not found", id))
                }
                DatasetRepositoryError::Unknown(cause) => {
                    tracing::error!("{:?}\n{}", cause, cause.backtrace());
                    Self::InternalServerError("Internal server error".to_string())
                }
            },
            DatasetServiceError::DataSource(ds_err) => {
                Self::UnprocessableEntity(ds_err.to_string())
            }
            DatasetServiceError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
                Self::InternalServerError("Internal server error".to_string())
            }
        }
    }
}
impl From<crate::domain::dataset::ports::DataSourceError> for ApiError {
    fn from(err: crate::domain::dataset::ports::DataSourceError) -> Self {
        match err {
            crate::domain::dataset::ports::DataSourceError::InvalidConnection(msg) => {
                Self::UnprocessableEntity(msg)
            }
            crate::domain::dataset::ports::DataSourceError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
                Self::InternalServerError("Internal server error".to_string())
            }
        }
    }
}

impl From<crate::domain::auth::service::LoginError> for ApiError {
    fn from(err: crate::domain::auth::service::LoginError) -> Self {
        match err {
            crate::domain::auth::service::LoginError::InvalidCredentials => {
                Self::UnprocessableEntity("Invalid credentials".to_string())
            }
            crate::domain::auth::service::LoginError::UserNotFound => {
                Self::NotFound("User not found".to_string())
            }
            _ => Self::InternalServerError("Internal server error".to_string()),
        }
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        tracing::error!("{:?}\n{}", err, err.backtrace());
        Self::InternalServerError("Internal server error".to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::UnprocessableEntity(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
}

impl From<crate::domain::user::models::User> for UserResponse {
    fn from(user: crate::domain::user::models::User) -> Self {
        Self {
            id: user.id.0,
            email: user.email,
            name: user.name,
        }
    }
}

#[derive(Serialize)]
pub struct ProjectResponse {
    pub id: uuid::Uuid,
    pub name: String,
}

impl From<crate::domain::project::models::Project> for ProjectResponse {
    fn from(project: crate::domain::project::models::Project) -> Self {
        Self {
            id: project.id.0,
            name: project.name,
        }
    }
}

#[derive(Serialize)]
pub struct ListProjectsResponse {
    pub projects: Vec<ProjectResponse>,
}