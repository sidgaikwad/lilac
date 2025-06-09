use crate::{
    model::{
        dataset::{Dataset, DatasetId},
        project::ProjectId,
    },
    ServiceError,
};

use super::Database;

impl Database {
    pub async fn get_dataset(&self, dataset_id: &DatasetId) -> Result<Dataset, ServiceError> {
        let id = dataset_id.inner();
        let dataset = sqlx::query!(
            // language=PostgreSQL
            r#"
            SELECT d.dataset_id, d.dataset_name, d.description, d.project_id, d.dataset_source
            FROM "datasets" d
            WHERE d.dataset_id = $1
        "#,
            id
        )
        .map(|row| Dataset {
            dataset_id: row.dataset_id.into(),
            dataset_name: row.dataset_name,
            description: row.description,
            project_id: row.project_id.into(),
            dataset_source: serde_json::from_value(row.dataset_source).unwrap_or_default(),
        })
        .fetch_one(&self.pool)
        .await?;
        Ok(dataset)
    }

    pub async fn list_datasets(
        &self,
        project_id: &ProjectId,
    ) -> Result<Vec<Dataset>, ServiceError> {
        let id = project_id.inner();
        let datasets = sqlx::query!(
            // language=PostgreSQL
            r#"
            SELECT d.dataset_id, d.dataset_name, d.description, d.project_id, d.dataset_source
            FROM "datasets" d
            WHERE d.project_id = $1
        "#,
            id
        )
        .map(|row| Dataset {
            dataset_id: row.dataset_id.into(),
            dataset_name: row.dataset_name,
            description: row.description,
            project_id: row.project_id.into(),
            dataset_source: serde_json::from_value(row.dataset_source).unwrap_or_default(),
        })
        .fetch_all(&self.pool)
        .await?;
        Ok(datasets)
    }

    pub async fn create_dataset(&self, dataset: Dataset) -> Result<DatasetId, ServiceError> {
        let dataset_id = sqlx::query!(
        // language=PostgreSQL
        r#"
            INSERT INTO "datasets" (dataset_id, dataset_name, description, project_id, dataset_source) VALUES ($1, $2, $3, $4, $5) RETURNING dataset_id
        "#,
        dataset.dataset_id.inner(),
        &dataset.dataset_name,
        dataset.description.as_ref(),
        &dataset.project_id.inner(),
        serde_json::to_value(dataset.dataset_source)?,
    )
    .map(|row| DatasetId::new(row.dataset_id))
    .fetch_one(&self.pool)
    .await?;
        Ok(dataset_id)
    }

    pub async fn delete_dataset(&self, dataset_id: &DatasetId) -> Result<(), ServiceError> {
        let id = dataset_id.inner();
        sqlx::query!(
            // language=PostgreSQL
            r#"
            DELETE FROM datasets WHERE dataset_id = $1;
        "#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
