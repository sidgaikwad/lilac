use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::domain::dataset::{
    models::{CreateDatasetRequest, Dataset, DatasetId},
    ports::{DatasetRepository, DatasetRepositoryError},
};

#[derive(Clone)]
pub struct PostgresDatasetRepository {
    pool: PgPool,
}

impl PostgresDatasetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

#[derive(sqlx::FromRow)]
struct DatasetRecord {
    dataset_id: uuid::Uuid,
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
            id: record.dataset_id.into(),
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
        req: &CreateDatasetRequest,
    ) -> Result<Dataset, DatasetRepositoryError> {
        let id = DatasetId::generate();

        let record = sqlx::query_as!(
            DatasetRecord,
            r#"
            INSERT INTO datasets (dataset_id, dataset_name, description, dataset_source)
            VALUES ($1, $2, $3, $4)
            RETURNING dataset_id, dataset_name, description, dataset_source, created_at, updated_at
            "#,
            id.inner(),
            req.name,
            req.description,
            serde_json::to_value(&req.source)
                .map_err(|e| DatasetRepositoryError::Unknown(anyhow::anyhow!(e)))?,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e: sqlx::Error| DatasetRepositoryError::Unknown(anyhow::anyhow!(e)))?;

        let dataset: Dataset = record.try_into().map_err(DatasetRepositoryError::Unknown)?;

        Ok(dataset)
    }

    async fn get_dataset_by_id(&self, id: &DatasetId) -> Result<Dataset, DatasetRepositoryError> {
        let record = sqlx::query_as!(
            DatasetRecord,
            r#"
            SELECT dataset_id, dataset_name, description, dataset_source, created_at, updated_at
            FROM datasets
            WHERE dataset_id = $1
            "#,
            id.inner()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DatasetRepositoryError::NotFound(id.to_string()),
            _ => DatasetRepositoryError::Unknown(anyhow::anyhow!(e)),
        })?;

        let dataset: Dataset = record.try_into().map_err(DatasetRepositoryError::Unknown)?;

        Ok(dataset)
    }

    async fn list_datasets(&self) -> Result<Vec<Dataset>, DatasetRepositoryError> {
        let records = sqlx::query_as!(
            DatasetRecord,
            r#"
            SELECT dataset_id, dataset_name, description, dataset_source, created_at, updated_at
            FROM datasets
            "#,
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

    async fn delete_dataset(&self, id: &DatasetId) -> Result<(), DatasetRepositoryError> {
        sqlx::query!("DELETE FROM datasets WHERE dataset_id = $1", id.inner())
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e: sqlx::Error| DatasetRepositoryError::Unknown(anyhow::anyhow!(e)))
    }
}
