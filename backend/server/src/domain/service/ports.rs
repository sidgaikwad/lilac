use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    service::models::{CreateService, Service, UpdateService},
    user::models::UserId,
};

#[async_trait]
pub trait ServiceRepository: Send + Sync {
    async fn create_service(
        &self,
        user_id: &UserId,
        service: &CreateService,
    ) -> anyhow::Result<Service>;
    async fn get_services_by_project_id(
        &self,
        user_id: &UserId,
        project_id: Uuid,
    ) -> anyhow::Result<Vec<Service>>;
    async fn get_service_by_id(
        &self,
        user_id: &UserId,
        service_id: Uuid,
    ) -> anyhow::Result<Option<Service>>;
    async fn update_service(
        &self,
        user_id: &UserId,
        service_id: Uuid,
        service: &UpdateService,
    ) -> anyhow::Result<Service>;
    async fn delete_service(&self, user_id: &UserId, service_id: Uuid) -> anyhow::Result<()>;
}

#[async_trait]
pub trait ServiceService: Send + Sync {
    async fn create_service(
        &self,
        user_id: &UserId,
        service: &CreateService,
    ) -> anyhow::Result<Service>;
    async fn get_services_by_project_id(
        &self,
        user_id: &UserId,
        project_id: Uuid,
    ) -> anyhow::Result<Vec<Service>>;
    async fn get_service_by_id(
        &self,
        user_id: &UserId,
        service_id: Uuid,
    ) -> anyhow::Result<Option<Service>>;
    async fn update_service(
        &self,
        user_id: &UserId,
        service_id: Uuid,
        service: &UpdateService,
    ) -> anyhow::Result<Service>;
    async fn delete_service(&self, user_id: &UserId, service_id: Uuid) -> anyhow::Result<()>;
}
