use sqlx::query;

use crate::{
    model::{
        organization::OrganizationId,
        service::{Service, ServiceId},
    },
    ServiceError,
};

use super::Database;

impl Database {
    pub async fn get_service(&self, service_id: &ServiceId) -> Result<Service, ServiceError> {
        let id = service_id.inner();
        let service= query!(
            r#"
                SELECT service_id, service_name, organization_id, service_type, service_configuration
                FROM "services" WHERE service_id = $1
            "#,
            id,
        )
        .map(|row| Service {
            service_id: row.service_id.into(),
            service_name: row.service_name,
            organization_id: row.organization_id.into(),
            service_type: serde_json::from_value(row.service_configuration).unwrap_or_default(),
        })
        .fetch_one(&self.pool)
        .await?;
        Ok(service)
    }

    pub async fn list_services(
        &self,
        organization_id: &OrganizationId,
    ) -> Result<Vec<Service>, ServiceError> {
        let id = organization_id.inner();
        let services= query!(
            r#"
                SELECT service_id, service_name, organization_id, service_type, service_configuration
                FROM "services" WHERE organization_id = $1
            "#,
            id,
        )
        .map(|row| Service {
            service_id: row.service_id.into(),
            service_name: row.service_name,
            organization_id: row.organization_id.into(),
            service_type: serde_json::from_value(row.service_configuration).unwrap_or_default(),
        })
        .fetch_all(&self.pool)
        .await?;
        Ok(services)
    }

    pub async fn create_service(&self, service: Service) -> Result<ServiceId, ServiceError> {
        let proj_id = sqlx::query!(
            r#"
                INSERT INTO "services" (service_id, service_name, organization_id, service_type, service_configuration) VALUES ($1, $2, $3, $4, $5) RETURNING service_id
            "#,
            service.service_id.inner(),
            &service.service_name,
            service.organization_id.inner(),
            service.service_type.get_type(),
            serde_json::to_value(service.service_type)?,
        )
        .map(|row| ServiceId::new(row.service_id))
        .fetch_one(&self.pool)
        .await?;
        Ok(proj_id)
    }

    pub async fn delete_service(&self, service_id: &ServiceId) -> Result<(), ServiceError> {
        let service_id_inner = service_id.inner();

        sqlx::query!(
            r#"
                DELETE FROM "services" WHERE service_id = $1
            "#,
            service_id_inner
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
