use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::{
    project::{
        ports::ProjectRepository,
    },
    service::{
        models::{CreateService, Service, UpdateService},
        ports::ServiceRepository,
    },
    user::models::UserId,
};

use super::project_repository::PostgresProjectRepository;

#[derive(Clone)]
pub struct PostgresServiceRepository {
    pool: PgPool,
    project_repo: PostgresProjectRepository,
}

impl PostgresServiceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: pool.clone(),
            project_repo: PostgresProjectRepository::new(pool),
        }
    }

    async fn check_permission(&self, user_id: &UserId, service: &Service) -> anyhow::Result<()> {
        if !self
            .project_repo
            .is_user_project_member(user_id, &service.project_id.into())
            .await?
        {
            return Err(anyhow::anyhow!("User is not a member of the project"));
        }
        Ok(())
    }
}

#[async_trait]
impl ServiceRepository for PostgresServiceRepository {
    async fn create_service(
        &self,
        user_id: &UserId,
        service: &CreateService,
    ) -> anyhow::Result<Service> {
        if !self
            .project_repo
            .is_user_project_member(user_id, &service.project_id.into())
            .await?
        {
            return Err(anyhow::anyhow!("User is not a member of the project"));
        }

        let result = sqlx::query_as!(
            Service,
            "INSERT INTO services (project_id, service_name, description, url, service_type) VALUES ($1, $2, $3, $4, 'unknown') RETURNING service_id as \"id!\", project_id, service_name as \"name!\", description as \"description?\", url as \"url!\", created_at, updated_at",
            service.project_id,
            service.name,
            service.description,
            service.url
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn get_services_by_project_id(
        &self,
        user_id: &UserId,
        project_id: uuid::Uuid,
    ) -> anyhow::Result<Vec<Service>> {
        if !self
            .project_repo
            .is_user_project_member(user_id, &project_id.into())
            .await?
        {
            return Err(anyhow::anyhow!("User is not a member of the project"));
        }

        let services = sqlx::query_as!(
            Service,
            "SELECT service_id as \"id!\", project_id, service_name as \"name!\", description as \"description?\", url as \"url!\", created_at, updated_at FROM services WHERE project_id = $1",
            project_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(services)
    }

    async fn get_service_by_id(
        &self,
        user_id: &UserId,
        service_id: uuid::Uuid,
    ) -> anyhow::Result<Option<Service>> {
        let service = sqlx::query_as!(
            Service,
            "SELECT service_id as \"id!\", project_id, service_name as \"name!\", description as \"description?\", url as \"url!\", created_at, updated_at FROM services WHERE service_id = $1",
            service_id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(s) = &service {
            self.check_permission(user_id, s).await?;
        }

        Ok(service)
    }

    async fn update_service(
        &self,
        user_id: &UserId,
        service_id: uuid::Uuid,
        service: &UpdateService,
    ) -> anyhow::Result<Service> {
        let existing_service = self.get_service_by_id(user_id, service_id).await?;

        if let Some(ref s) = existing_service {
            self.check_permission(user_id, s).await?;
        } else {
            return Err(anyhow::anyhow!("Service not found"));
        }

        let updated = sqlx::query_as!(
            Service,
            "UPDATE services SET service_name = COALESCE($1, service_name), description = COALESCE($2, description), url = COALESCE($3, url) WHERE service_id = $4 RETURNING service_id as \"id!\", project_id, service_name as \"name!\", description as \"description?\", url as \"url!\", created_at, updated_at",
            service.name,
            service.description,
            service.url,
            service_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated)
    }

    async fn delete_service(&self, user_id: &UserId, service_id: uuid::Uuid) -> anyhow::Result<()> {
        let service = self.get_service_by_id(user_id, service_id).await?;

        if let Some(ref s) = service {
            self.check_permission(user_id, s).await?;
        }

        sqlx::query!("DELETE FROM services WHERE service_id = $1", service_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}