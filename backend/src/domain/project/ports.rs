use async_trait::async_trait;
use thiserror::Error;

use super::models::{CreateProjectRequest, Project, ProjectId};

#[derive(Debug, Error)]
pub enum ProjectRepositoryError {
    #[error("project with {field} {value} already exists")]
    Duplicate { field: String, value: String },
    #[error("project with id {0} not found")]
    NotFound(String),
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
    async fn list_projects(&self) -> Result<Vec<Project>, ProjectRepositoryError>;
    async fn delete_project(&self, id: &ProjectId) -> Result<(), ProjectRepositoryError>;
}
