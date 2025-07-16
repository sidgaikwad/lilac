use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    project::ports::ProjectRepository,
    service::{
        models::{CreateService, Service, UpdateService},
        ports::{ServiceRepository, ServiceService},
    },
    user::models::UserId,
};

pub struct ServiceServiceImpl {
    repository: Arc<dyn ServiceRepository>,
    project_repo: Arc<dyn ProjectRepository>,
}

impl ServiceServiceImpl {
    pub fn new(
        repository: Arc<dyn ServiceRepository>,
        project_repo: Arc<dyn ProjectRepository>,
    ) -> Self {
        Self {
            repository,
            project_repo,
        }
    }
}

#[async_trait]
impl ServiceService for ServiceServiceImpl {
    async fn create_service(
        &self,
        user_id: &UserId,
        service: &CreateService,
    ) -> anyhow::Result<Service> {
        self.project_repo
            .is_user_project_member(user_id, &service.project_id.into())
            .await?;
        self.repository.create_service(user_id, service).await
    }

    async fn get_services_by_project_id(
        &self,
        user_id: &UserId,
        project_id: Uuid,
    ) -> anyhow::Result<Vec<Service>> {
        self.project_repo
            .is_user_project_member(user_id, &project_id.into())
            .await?;
        self.repository
            .get_services_by_project_id(user_id, project_id)
            .await
    }

    async fn get_service_by_id(
        &self,
        user_id: &UserId,
        service_id: Uuid,
    ) -> anyhow::Result<Option<Service>> {
        let service = self
            .repository
            .get_service_by_id(user_id, service_id)
            .await?;
        if let Some(service) = &service {
            self.project_repo
                .is_user_project_member(user_id, &service.project_id.into())
                .await?;
        }
        Ok(service)
    }

    async fn update_service(
        &self,
        user_id: &UserId,
        service_id: Uuid,
        service: &UpdateService,
    ) -> anyhow::Result<Service> {
        let existing_service = self
            .repository
            .get_service_by_id(user_id, service_id)
            .await?;
        if let Some(existing_service) = existing_service {
            self.project_repo
                .is_user_project_member(user_id, &existing_service.project_id.into())
                .await?;
        }
        self.repository
            .update_service(user_id, service_id, service)
            .await
    }

    async fn delete_service(&self, user_id: &UserId, service_id: Uuid) -> anyhow::Result<()> {
        let service = self
            .repository
            .get_service_by_id(user_id, service_id)
            .await?;
        if let Some(service) = service {
            self.project_repo
                .is_user_project_member(user_id, &service.project_id.into())
                .await?;
        }
        self.repository.delete_service(user_id, service_id).await
    }
}
