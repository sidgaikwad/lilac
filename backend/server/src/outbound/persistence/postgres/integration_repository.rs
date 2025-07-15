use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    integration::{
        models::{
            AWSIntegration, CreateAWSIntegrationRequest, Integration, IntegrationId,
        },
        ports::{IntegrationRepository, IntegrationRepositoryError},
    },
    project::{
        models::ProjectId,
        ports::ProjectRepository,
    },
    user::models::UserId,
};

use super::project_repository::PostgresProjectRepository;

#[derive(Clone)]
pub struct PostgresIntegrationRepository {
    pool: PgPool,
    project_repo: PostgresProjectRepository,
}

impl PostgresIntegrationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: pool.clone(),
            project_repo: PostgresProjectRepository::new(pool),
        }
    }
}

#[async_trait]
impl IntegrationRepository for PostgresIntegrationRepository {
    async fn create_aws_integration(
        &self,
        user_id: &UserId,
        req: &CreateAWSIntegrationRequest,
    ) -> Result<AWSIntegration, IntegrationRepositoryError> {
        self.project_repo
            .is_user_project_member(user_id, &req.project_id)
            .await
            .map_err(|e| IntegrationRepositoryError::Unknown(e.into()))?;

        let id = IntegrationId(uuid::Uuid::new_v4());
        let external_id = format!("Lilac-{}", uuid::Uuid::new_v4().as_simple());

        let aws_integration = AWSIntegration {
            id,
            project_id: req.project_id.clone(),
            role_arn: req.role_arn.clone(),
            external_id,
        };

        let aws_integration_value = serde_json::to_value(&aws_integration)
            .map_err(|e| IntegrationRepositoryError::Unknown(anyhow::anyhow!(e)))?;

        sqlx::query!(
            r#"
            UPDATE projects
            SET aws_integration = $1
            WHERE project_id = $2
            "#,
            aws_integration_value,
            req.project_id.0,
        )
        .execute(&self.pool)
        .await
        .map_err(|e: sqlx::Error| IntegrationRepositoryError::Unknown(anyhow::anyhow!(e)))?;

        Ok(aws_integration)
    }

    async fn get_integrations_by_project_id(
        &self,
        user_id: &UserId,
        project_id: &ProjectId,
    ) -> Result<Vec<Integration>, IntegrationRepositoryError> {
        self.project_repo
            .is_user_project_member(user_id, project_id)
            .await
            .map_err(|e| IntegrationRepositoryError::Unknown(e.into()))?;

        let aws_integrations_json: Vec<(serde_json::Value,)> = sqlx::query_as(
            r#"
            SELECT aws_integration
            FROM projects
            WHERE project_id = $1 AND aws_integration IS NOT NULL
            "#,
        )
        .bind(project_id.0)
        .fetch_all(&self.pool)
        .await
        .map_err(|e: sqlx::Error| IntegrationRepositoryError::Unknown(anyhow::anyhow!(e)))?;

        let integrations = aws_integrations_json
            .into_iter()
            .map(|(json,)| {
                serde_json::from_value(json)
                    .map(Integration::AWS)
                    .map_err(|e| IntegrationRepositoryError::Unknown(anyhow::anyhow!(e)))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(integrations)
    }

    async fn get_integration_by_id(
        &self,
        user_id: &UserId,
        integration_id: &IntegrationId,
    ) -> Result<Integration, IntegrationRepositoryError> {
        let (project_id, aws_integration_json): (Uuid, serde_json::Value) = sqlx::query_as(
            r#"
            SELECT project_id, aws_integration
            FROM projects
            WHERE (aws_integration->>'id')::uuid = $1
            "#,
        )
        .bind(integration_id.0)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                IntegrationRepositoryError::NotFound(integration_id.0.to_string())
            }
            _ => IntegrationRepositoryError::Unknown(anyhow::anyhow!(e)),
        })?;

        let project_id = ProjectId(project_id);

        self.project_repo
            .is_user_project_member(user_id, &project_id)
            .await
            .map_err(|e| IntegrationRepositoryError::Unknown(e.into()))?;

        let aws_integration: AWSIntegration = serde_json::from_value(aws_integration_json)
            .map_err(|e| IntegrationRepositoryError::Unknown(anyhow::anyhow!(e)))?;

        Ok(Integration::AWS(aws_integration))
    }
}