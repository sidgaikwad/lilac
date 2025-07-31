use async_trait::async_trait;
use std::sync::Arc;

use super::{
    models::{CreateProjectRequest, Project, ProjectId},
    ports::{ProjectRepository, ProjectRepositoryError},
};

#[derive(Debug, thiserror::Error)]
pub enum ProjectServiceError {
    #[error("invalid permissions")]
    InvalidPermissions,
    #[error("project with {field} {value} already exists")]
    ProjectExists { field: String, value: String },
    #[error("project {0} not found")]
    ProjectNotFound(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<ProjectRepositoryError> for ProjectServiceError {
    fn from(error: ProjectRepositoryError) -> Self {
        match error {
            ProjectRepositoryError::Duplicate { field, value } => {
                Self::ProjectExists { field, value }
            }
            ProjectRepositoryError::NotFound(id) => Self::ProjectNotFound(id),
            ProjectRepositoryError::Unknown(error) => Self::Unknown(error),
        }
    }
}

#[async_trait]
pub trait ProjectService: Send + Sync {
    async fn create_project(
        &self,
        req: &CreateProjectRequest,
    ) -> Result<Project, ProjectServiceError>;
    async fn get_project_by_id(&self, id: &ProjectId) -> Result<Project, ProjectServiceError>;
    async fn list_projects(&self) -> Result<Vec<Project>, ProjectServiceError>;
    async fn delete_project(&self, id: &ProjectId) -> Result<(), ProjectServiceError>;
}

#[derive(Clone)]
pub struct ProjectServiceImpl<R: ProjectRepository> {
    repo: Arc<R>,
}

impl<R: ProjectRepository> ProjectServiceImpl<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: ProjectRepository> ProjectService for ProjectServiceImpl<R> {
    async fn create_project(
        &self,
        req: &CreateProjectRequest,
    ) -> Result<Project, ProjectServiceError> {
        Ok(self.repo.create_project(req).await?)
    }

    async fn get_project_by_id(&self, id: &ProjectId) -> Result<Project, ProjectServiceError> {
        Ok(self.repo.get_project_by_id(id).await?)
    }

    async fn list_projects(&self) -> Result<Vec<Project>, ProjectServiceError> {
        Ok(self.repo.list_projects().await?)
    }

    async fn delete_project(&self, id: &ProjectId) -> Result<(), ProjectServiceError> {
        Ok(self.repo.delete_project(id).await?)
    }
}
