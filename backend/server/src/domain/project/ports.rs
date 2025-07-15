use async_trait::async_trait;
use thiserror::Error;

use crate::domain::user::models::UserId;

use super::models::{CreateProjectRequest, Project, ProjectId};

#[derive(Debug, Error)]
pub enum ProjectRepositoryError {
    #[error("{0} with value {1} already exists")]
    Duplicate(String, String),
    #[error("project with id {0} not found")]
    NotFound(String),
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn create_project(
        &self,
        req: &CreateProjectRequest,
    ) -> Result<Project, ProjectRepositoryError>;
    async fn get_project_by_id(&self, id: &ProjectId) -> Result<Project, ProjectRepositoryError>;
    async fn list_projects_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<Project>, ProjectRepositoryError>;
    async fn delete_project(&self, id: &ProjectId) -> Result<(), ProjectRepositoryError>;
    async fn is_user_project_member(
        &self,
        user_id: &UserId,
        project_id: &ProjectId,
    ) -> Result<bool, ProjectRepositoryError>;
    async fn add_user_to_project(
        &self,
        project_id: &ProjectId,
        user_id: &UserId,
        role: &str,
    ) -> Result<(), ProjectRepositoryError>;
}