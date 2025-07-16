use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::domain::{
    dataset::{
        models::{CreateDatasetRequest, Dataset, DatasetId},
        ports::{DatasetRepository, DatasetRepositoryError},
    },
    project::{models::ProjectId, ports::ProjectRepository},
    user::models::UserId,
};

use super::project_repository::PostgresProjectRepository;

#[derive(Clone)]
pub struct PostgresDatasetRepository {
    pool: PgPool,
    project_repo: PostgresProjectRepository,
}

impl PostgresDatasetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: pool.clone(),
            project_repo: PostgresProjectRepository::new(pool),
        }
    }
}

async fn ensure_user_is_project_member(
    project_repo: &PostgresProjectRepository,
    user_id: &UserId,
    project_id: &ProjectId,
) -> Result<(), DatasetRepositoryError> {
    let is_member = project_repo
        .is_user_project_member(user_id, project_id)
        .await
        .map_err(|e| DatasetRepositoryError::Unknown(e.into()))?;

    if !is_member {
        return Err(DatasetRepositoryError::Unknown(anyhow::anyhow!(
            "User is not a member of the project"
        )));
    }
    Ok(())
}

#[derive(sqlx::FromRow)]
struct DatasetRecord {
    dataset_id: uuid::Uuid,
    project_id: uuid::Uuid,
    dataset_name: String,
    description: Option<String>,
    dataset_source: serde_json::Value,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

use crate::domain::dataset::models::DatasetSource;

impl TryFrom<DatasetRecord> for Dataset {
    type Error = anyhow::Error;

    fn try_from(record: DatasetRecord) -> Result<Self, Self::Error> {
        let source: DatasetSource = serde_json::from_value(record.dataset_source)?;

        Ok(Self {
            id: DatasetId(record.dataset_id),
            project_id: ProjectId(record.project_id),
            name: record.dataset_name,
            description: record.description,
            source,
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
    }
}

#[async_trait]
impl DatasetRepository for PostgresDatasetRepository {
    async fn create_dataset(
        &self,
        user_id: &UserId,
        req: &CreateDatasetRequest,
    ) -> Result<Dataset, DatasetRepositoryError> {
        ensure_user_is_project_member(&self.project_repo, user_id, &req.project_id).await?;

        let id = DatasetId(uuid::Uuid::new_v4());

        let record = sqlx::query_as!(
            DatasetRecord,
            r#"
            INSERT INTO datasets (dataset_id, project_id, dataset_name, description, dataset_source)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING dataset_id, project_id, dataset_name, description, dataset_source, created_at, updated_at
            "#,
            id.0,
            req.project_id.0,
            req.name,
            req.description,
            serde_json::to_value(&req.source)
                .map_err(|e| DatasetRepositoryError::Unknown(anyhow::anyhow!(e)))?,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e: sqlx::Error| DatasetRepositoryError::Unknown(anyhow::anyhow!(e)))?;

        let dataset: Dataset = record
            .try_into()
            .map_err(DatasetRepositoryError::Unknown)?;

        Ok(dataset)
    }

    async fn get_dataset_by_id(
        &self,
        user_id: &UserId,
        id: &DatasetId,
    ) -> Result<Dataset, DatasetRepositoryError> {
        let record = sqlx::query_as!(
            DatasetRecord,
            r#"
            SELECT dataset_id, project_id, dataset_name, description, dataset_source, created_at, updated_at
            FROM datasets
            WHERE dataset_id = $1
            "#,
            id.0
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DatasetRepositoryError::NotFound(id.0.to_string()),
            _ => DatasetRepositoryError::Unknown(anyhow::anyhow!(e)),
        })?;

        ensure_user_is_project_member(&self.project_repo, user_id, &ProjectId(record.project_id))
            .await?;

        let dataset: Dataset = record
            .try_into()
            .map_err(DatasetRepositoryError::Unknown)?;

        Ok(dataset)
    }

    async fn list_datasets_by_project_id(
        &self,
        user_id: &UserId,
        project_id: &ProjectId,
    ) -> Result<Vec<Dataset>, DatasetRepositoryError> {
        ensure_user_is_project_member(&self.project_repo, user_id, project_id).await?;

        let records = sqlx::query_as!(
            DatasetRecord,
            r#"
            SELECT dataset_id, project_id, dataset_name, description, dataset_source, created_at, updated_at
            FROM datasets
            WHERE project_id = $1
            "#,
            project_id.0
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e: sqlx::Error| DatasetRepositoryError::Unknown(anyhow::anyhow!(e)))?;

        let datasets: Vec<Dataset> = records
            .into_iter()
            .map(|r| r.try_into())
            .collect::<Result<Vec<_>, _>>()
            .map_err(DatasetRepositoryError::Unknown)?;

        Ok(datasets)
    }

    async fn delete_dataset(
        &self,
        user_id: &UserId,
        id: &DatasetId,
    ) -> Result<(), DatasetRepositoryError> {
        let dataset = self.get_dataset_by_id(user_id, id).await?;
        ensure_user_is_project_member(&self.project_repo, user_id, &dataset.project_id).await?;

        sqlx::query!("DELETE FROM datasets WHERE dataset_id = $1", id.0)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e: sqlx::Error| DatasetRepositoryError::Unknown(anyhow::anyhow!(e)))
    }
}
