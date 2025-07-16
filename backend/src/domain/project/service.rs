use async_trait::async_trait;
use std::sync::Arc;
use validator::Validate;

use crate::domain::user::models::UserId;

use super::{
    models::{CreateProjectRequest, Project, ProjectId},
    ports::{ProjectRepository, ProjectRepositoryError},
};

#[async_trait]
pub trait ProjectService: Send + Sync {
    async fn create_project(
        &self,
        user_id: &UserId,
        req: &CreateProjectRequest,
    ) -> Result<Project, ProjectRepositoryError>;
    async fn get_project_by_id(
        &self,
        user_id: &UserId,
        id: &ProjectId,
    ) -> Result<Project, ProjectRepositoryError>;
    async fn list_projects_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<Project>, ProjectRepositoryError>;
    async fn delete_project(
        &self,
        user_id: &UserId,
        id: &ProjectId,
    ) -> Result<(), ProjectRepositoryError>;
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
        user_id: &UserId,
        req: &CreateProjectRequest,
    ) -> Result<Project, ProjectRepositoryError> {
        req.validate()
            .map_err(|e| ProjectRepositoryError::InvalidInput(e.to_string()))?;

        let mut req = req.clone();
        if req.owner_id.is_none() {
            req.owner_id = Some(*user_id);
        }

        self.repo.create_project(&req).await
    }

    async fn get_project_by_id(
        &self,
        user_id: &UserId,
        id: &ProjectId,
    ) -> Result<Project, ProjectRepositoryError> {
        if !self.repo.is_user_project_member(user_id, id).await? {
            return Err(ProjectRepositoryError::Unauthorized);
        }
        self.repo.get_project_by_id(id).await
    }

    async fn list_projects_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<Project>, ProjectRepositoryError> {
        self.repo.list_projects_by_user_id(user_id).await
    }

    async fn delete_project(
        &self,
        user_id: &UserId,
        id: &ProjectId,
    ) -> Result<(), ProjectRepositoryError> {
        if !self.repo.is_user_project_member(user_id, id).await? {
            return Err(ProjectRepositoryError::Unauthorized);
        }
        self.repo.delete_project(id).await
    }
}
