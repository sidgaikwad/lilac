use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::domain::credentials::{
    models::{CreateCredentialRequest, Credential, CredentialId, Credentials},
    ports::{CredentialRepository, CredentialRepositoryError},
};

#[derive(Clone)]
pub struct PostgresCredentialRepository {
    pool: PgPool,
}

impl PostgresCredentialRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct CredentialRecord {
    credential_id: uuid::Uuid,
    credential_name: String,
    credential_description: Option<String>,
    credentials: serde_json::Value,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<CredentialRecord> for Credential {
    type Error = anyhow::Error;

    fn try_from(record: CredentialRecord) -> Result<Self, Self::Error> {
        let credentials: Credentials = serde_json::from_value(record.credentials)?;

        Ok(Self {
            id: CredentialId(record.credential_id),
            name: record.credential_name,
            description: record.credential_description,
            credentials,
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
    }
}

#[async_trait]
impl CredentialRepository for PostgresCredentialRepository {
    async fn create_credential(
        &self,
        req: &CreateCredentialRequest,
    ) -> Result<Credential, CredentialRepositoryError> {
        let credential_id = CredentialId(uuid::Uuid::new_v4());

        let record = sqlx::query_as!(
            CredentialRecord,
            "INSERT INTO credentials (credential_id, credential_name, credential_description, credentials) VALUES ($1, $2, $3, $4) RETURNING credential_id, credential_name, credential_description, credentials, created_at, updated_at",
            credential_id.0,
            req.name,
            req.description,
            serde_json::to_value(&req.credentials)
                .map_err(|e| CredentialRepositoryError::Unknown(anyhow::anyhow!(e)))?,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| CredentialRepositoryError::Unknown(e.into()))?;

        Ok(record.try_into()?)
    }

    async fn get_credential_by_id(
        &self,
        id: &CredentialId,
    ) -> Result<Credential, CredentialRepositoryError> {
        let record = sqlx::query_as!(
            CredentialRecord,
            r#"
            SELECT credential_id, credential_name, credential_description, credentials, created_at, updated_at
            FROM credentials
            WHERE credential_id = $1
            "#,
            id.0
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => CredentialRepositoryError::NotFound(id.0.to_string()),
            _ => CredentialRepositoryError::Unknown(anyhow::anyhow!(e)),
        })?;

        let credential: Credential = record
            .try_into()
            .map_err(CredentialRepositoryError::Unknown)?;

        Ok(credential)
    }

    async fn list_credentials(&self) -> Result<Vec<Credential>, CredentialRepositoryError> {
        let records = sqlx::query_as!(
            CredentialRecord,
            r#"
            SELECT credential_id, credential_name, credential_description, credentials, created_at, updated_at
            FROM credentials
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e: sqlx::Error| CredentialRepositoryError::Unknown(anyhow::anyhow!(e)))?;

        let credentials: Vec<Credential> = records
            .into_iter()
            .map(|r| r.try_into())
            .collect::<Result<Vec<_>, _>>()
            .map_err(CredentialRepositoryError::Unknown)?;
        Ok(credentials)
    }

    async fn delete_credential(&self, id: &CredentialId) -> Result<(), CredentialRepositoryError> {
        sqlx::query!("DELETE FROM credentials WHERE credential_id = $1", id.0)
            .execute(&self.pool)
            .await
            .map_err(|e: sqlx::Error| CredentialRepositoryError::Unknown(anyhow::anyhow!(e)))?;
        Ok(())
    }
}
