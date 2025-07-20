use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::domain::cluster::{
    models::{Cluster, ClusterConfig, ClusterId, CreateClusterRequest},
    ports::{ClusterRepository, ClusterRepositoryError},
};

#[derive(Clone)]
pub struct PostgresClusterRepository {
    pool: PgPool,
}

impl PostgresClusterRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct ClusterRecord {
    cluster_id: uuid::Uuid,
    cluster_name: String,
    cluster_description: Option<String>,
    credential_id: uuid::Uuid,
    cluster_config: serde_json::Value,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<ClusterRecord> for Cluster {
    type Error = anyhow::Error;

    fn try_from(record: ClusterRecord) -> Result<Self, Self::Error> {
        let cluster_config: ClusterConfig = serde_json::from_value(record.cluster_config)?;

        Ok(Self {
            id: record.cluster_id.into(),
            name: record.cluster_name,
            description: record.cluster_description,
            credential_id: record.credential_id.into(),
            cluster_config,
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
    }
}

#[async_trait]
impl ClusterRepository for PostgresClusterRepository {
    async fn create_cluster(
        &self,
        req: &CreateClusterRequest,
    ) -> Result<Cluster, ClusterRepositoryError> {
        let cluster_id = ClusterId(uuid::Uuid::new_v4());

        let record = sqlx::query_as!(
            ClusterRecord,
            "INSERT INTO clusters (cluster_id, cluster_name, cluster_description, credential_id, cluster_config) VALUES ($1, $2, $3, $4, $5) RETURNING cluster_id, cluster_name, cluster_description, credential_id, cluster_config, created_at, updated_at",
            cluster_id.0,
            req.name,
            req.description,
            req.credential_id.inner(),
            serde_json::to_value(&req.cluster_config)
                .map_err(|e| ClusterRepositoryError::Unknown(anyhow::anyhow!(e)))?,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ClusterRepositoryError::Unknown(e.into()))?;

        Ok(record.try_into()?)
    }

    async fn get_cluster_by_id(&self, id: &ClusterId) -> Result<Cluster, ClusterRepositoryError> {
        let record = sqlx::query_as!(
            ClusterRecord,
            r#"
            SELECT cluster_id, cluster_name, cluster_description, credential_id, cluster_config, created_at, updated_at
            FROM clusters
            WHERE cluster_id = $1
            "#,
            id.0
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ClusterRepositoryError::NotFound(id.0.to_string()),
            _ => ClusterRepositoryError::Unknown(anyhow::anyhow!(e)),
        })?;

        let cluster: Cluster = record.try_into().map_err(ClusterRepositoryError::Unknown)?;

        Ok(cluster)
    }

    async fn list_clusters(&self) -> Result<Vec<Cluster>, ClusterRepositoryError> {
        let records = sqlx::query_as!(
            ClusterRecord,
            r#"
            SELECT cluster_id, cluster_name, cluster_description, credential_id, cluster_config, created_at, updated_at
            FROM clusters
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e: sqlx::Error| ClusterRepositoryError::Unknown(anyhow::anyhow!(e)))?;

        let clusters: Vec<Cluster> = records
            .into_iter()
            .map(|r| r.try_into())
            .collect::<Result<Vec<_>, _>>()
            .map_err(ClusterRepositoryError::Unknown)?;
        Ok(clusters)
    }

    async fn delete_cluster(&self, id: &ClusterId) -> Result<(), ClusterRepositoryError> {
        sqlx::query!("DELETE FROM clusters WHERE cluster_id = $1", id.0)
            .execute(&self.pool)
            .await
            .map_err(|e: sqlx::Error| ClusterRepositoryError::Unknown(anyhow::anyhow!(e)))?;
        Ok(())
    }
}
