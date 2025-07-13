use sqlx::query;

use crate::{
    database::DatabaseError,
    model::{
        project::ProjectId,
        service::{Service, ServiceId},
    },
};

use super::Database;

impl Database {
    pub async fn get_service(&self, service_id: &ServiceId) -> Result<Service, DatabaseError> {
        let id = service_id.inner();
        let service= query!(
            r#"
                SELECT service_id, service_name, project_id, service_type, service_configuration, url
                FROM "services" WHERE service_id = $1
            "#,
            id,
        )
        .map(|row| Service {
            service_id: row.service_id.into(),
            service_name: row.service_name,
            project_id: row.project_id.into(),
            service_type: serde_json::from_value(row.service_configuration).unwrap_or_else(|e| {
                tracing::warn!("Failed to deserialize service_configuration: {}", e);
                Default::default()
            }),
            url: row.url,
        })
        .fetch_one(&self.pool)
        .await?;
        Ok(service)
    }

    pub async fn list_services(
        &self,
        project_id: &ProjectId,
    ) -> Result<Vec<Service>, DatabaseError> {
        let id = project_id.inner();
        let services= query!(
            r#"
                SELECT service_id, service_name, project_id, service_type, service_configuration, url
                FROM "services" WHERE project_id = $1
            "#,
            id,
        )
        .map(|row| Service {
            service_id: row.service_id.into(),
            service_name: row.service_name,
            project_id: row.project_id.into(),
            service_type: serde_json::from_value(row.service_configuration).unwrap_or_else(|e| {
                tracing::warn!("Failed to deserialize service_configuration: {}", e);
                Default::default()
            }),
            url: row.url,
        })
        .fetch_all(&self.pool)
        .await?;
        Ok(services)
    }

    pub async fn create_service(&self, service: Service) -> Result<ServiceId, DatabaseError> {
        let proj_id = sqlx::query!(
            r#"
                INSERT INTO "services" (service_id, service_name, project_id, service_type, service_configuration, url) VALUES ($1, $2, $3, $4, $5, $6) RETURNING service_id
            "#,
            service.service_id.inner(),
            &service.service_name,
            service.project_id.inner(),
            service.service_type.get_type(),
            serde_json::to_value(service.service_type)?,
            service.url,
        )
        .map(|row| ServiceId::new(row.service_id))
        .fetch_one(&self.pool)
        .await?;
        Ok(proj_id)
    }

    pub async fn delete_service(&self, service_id: &ServiceId) -> Result<(), DatabaseError> {
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
