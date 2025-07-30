use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::{
    domain::{
        cluster::{
            models::{
                Architecture, Cluster, ClusterId, ClusterNode, Cpu, CpuManufacturer,
                CreateClusterRequest, Gpu, GpuManufacturer, GpuModel, NodeId, NodeStatus,
                UpdateNodeStatusRequest,
            },
            ports::{
                ClusterApiKeyRepository, ClusterApiKeyRepositoryError, ClusterRepository,
                ClusterRepositoryError,
            },
        },
        user::models::{ApiKey, ApiKeyId},
    },
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
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<ClusterRecord> for Cluster {
    fn from(record: ClusterRecord) -> Self {
        Self {
            id: record.cluster_id.into(),
            name: record.cluster_name,
            description: record.cluster_description,
            created_at: record.created_at,
            updated_at: record.updated_at,
        }
    }
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "node_status", rename_all = "snake_case")]
pub enum NodeStatusRecord {
    Available,
    Busy,
}

impl From<NodeStatus> for NodeStatusRecord {
    fn from(value: NodeStatus) -> Self {
        match value {
            NodeStatus::Available => NodeStatusRecord::Available,
            NodeStatus::Busy => NodeStatusRecord::Busy,
        }
    }
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "cpu_configuration")]
pub struct CpuConfiguration {
    manufacturer: String,
    architecture: String,
    millicores: i32,
}

impl From<Cpu> for CpuConfiguration {
    fn from(value: Cpu) -> Self {
        Self {
            manufacturer: value.manufacturer.to_string(),
            architecture: value.architecture.to_string(),
            millicores: value.millicores,
        }
    }
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "gpu_configuration")]
pub struct GpuConfiguration {
    manufacturer: String,
    model_name: String,
    memory_mb: i32,
    count: i32,
}

impl From<Gpu> for GpuConfiguration {
    fn from(value: Gpu) -> Self {
        Self {
            manufacturer: value.manufacturer.to_string(),
            model_name: value.model.to_string(),
            memory_mb: value.memory_mb,
            count: value.count,
        }
    }
}


#[derive(sqlx::FromRow)]
struct ClusterNodeRecord {
    node_id: uuid::Uuid,
    cluster_id: uuid::Uuid,
    node_status: NodeStatusRecord,
    heartbeat_timestamp: DateTime<Utc>,
    memory_mb: i32,
    cpu: CpuConfiguration,
    gpu: Option<GpuConfiguration>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    assigned_job_id: Option<uuid::Uuid>,
    reported_job_id: Option<uuid::Uuid>,
}

impl TryFrom<ClusterNodeRecord> for ClusterNode {
    type Error = anyhow::Error;

    fn try_from(record: ClusterNodeRecord) -> Result<Self, Self::Error> {
        Ok(Self {
            id: record.node_id.into(),
            cluster_id: record.cluster_id.into(),
            node_status: match record.node_status {
                NodeStatusRecord::Available => NodeStatus::Available,
                NodeStatusRecord::Busy => NodeStatus::Busy,
            },
            heartbeat_timestamp: record.heartbeat_timestamp,
            memory_mb: record.memory_mb,
            cpu: Cpu {
                manufacturer: CpuManufacturer::try_from(record.cpu.manufacturer.as_str())?,
                architecture: Architecture::try_from(record.cpu.architecture.as_str())?,
                millicores: record.cpu.millicores,
            },
            gpu: match record.gpu {
                Some(gpu) => Some(Gpu {
                    manufacturer: GpuManufacturer::try_from(gpu.manufacturer.as_str())?,
                    model: GpuModel::try_from(gpu.model_name.as_str())?,
                    count: gpu.count,
                    memory_mb: gpu.memory_mb,
                }),
                None => None,
            },
            created_at: record.created_at,
            updated_at: record.updated_at,
            assigned_job_id: record.assigned_job_id.map(Into::into),
            reported_job_id: record.reported_job_id.map(Into::into),
        })
    }
}

#[derive(sqlx::FromRow)]
struct ApiKeyRecord {
    id: uuid::Uuid,
    user_id: Option<uuid::Uuid>,
    cluster_id: Option<uuid::Uuid>,
    prefix: String,
    key_hash: String,
    created_at: DateTime<Utc>,
    last_used_at: Option<DateTime<Utc>>,
    expires_at: Option<DateTime<Utc>>,
}

impl From<ApiKeyRecord> for ApiKey {
    fn from(record: ApiKeyRecord) -> Self {
        Self {
            id: record.id.into(),
            user_id: record.user_id.map(|v| v.into()),
            cluster_id: record.cluster_id.map(|v| v.into()),
            prefix: record.prefix,
            key_hash: record.key_hash,
            created_at: record.created_at,
            last_used_at: record.last_used_at,
            expires_at: record.expires_at,
        }
    }
}

#[async_trait]
impl ClusterRepository for PostgresClusterRepository {
    async fn create_cluster(
        &self,
        req: &CreateClusterRequest,
    ) -> Result<Cluster, ClusterRepositoryError> {
        let record = sqlx::query_as!(
            ClusterRecord,
            "INSERT INTO clusters (cluster_name, cluster_description) VALUES ($1, $2) RETURNING cluster_id, cluster_name, cluster_description, created_at, updated_at",
            req.name,
            req.description,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ClusterRepositoryError::Unknown(e.into()))?;

        Ok(record.into())
    }

    async fn get_cluster_by_id(&self, id: &ClusterId) -> Result<Cluster, ClusterRepositoryError> {
        let record = sqlx::query_as!(
            ClusterRecord,
            r#"
            SELECT cluster_id, cluster_name, cluster_description, created_at, updated_at
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

        let cluster: Cluster = record.into();

        Ok(cluster)
    }

    async fn list_clusters(&self) -> Result<Vec<Cluster>, ClusterRepositoryError> {
        let records = sqlx::query_as!(
            ClusterRecord,
            r#"
            SELECT cluster_id, cluster_name, cluster_description, created_at, updated_at
            FROM clusters
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e: sqlx::Error| ClusterRepositoryError::Unknown(anyhow::anyhow!(e)))?;

        let clusters: Vec<Cluster> = records.into_iter().map(|r| r.into()).collect();
        Ok(clusters)
    }

    async fn delete_cluster(&self, id: &ClusterId) -> Result<(), ClusterRepositoryError> {
        sqlx::query!("DELETE FROM clusters WHERE cluster_id = $1", id.0)
            .execute(&self.pool)
            .await
            .map_err(|e: sqlx::Error| ClusterRepositoryError::Unknown(anyhow::anyhow!(e)))?;
        Ok(())
    }

    async fn list_cluster_nodes(
        &self,
        id: &ClusterId,
    ) -> Result<Vec<ClusterNode>, ClusterRepositoryError> {
        let records = sqlx::query_as!(
            ClusterNodeRecord,
            r#"
            SELECT node_id, cluster_id, node_status as "node_status: NodeStatusRecord", heartbeat_timestamp, memory_mb, cpu as "cpu: CpuConfiguration", gpu as "gpu: GpuConfiguration", created_at, updated_at, assigned_job_id, reported_job_id
            FROM cluster_nodes
            WHERE cluster_id = $1
            "#,
            id.0,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e: sqlx::Error| ClusterRepositoryError::Unknown(anyhow::anyhow!(e)))?;

        let clusters: Vec<ClusterNode> = records
            .into_iter()
            .map(|r| r.try_into())
            .collect::<Result<Vec<_>, _>>()
            .map_err(ClusterRepositoryError::Unknown)?;
        Ok(clusters)
    }
    async fn get_cluster_node_by_id(
        &self,
        id: &NodeId,
    ) -> Result<ClusterNode, ClusterRepositoryError> {
        let record = sqlx::query_as!(
            ClusterNodeRecord,
            r#"
            SELECT node_id, cluster_id, node_status as "node_status: NodeStatusRecord", heartbeat_timestamp, memory_mb, cpu as "cpu: CpuConfiguration", gpu as "gpu: GpuConfiguration", created_at, updated_at, assigned_job_id, reported_job_id
            FROM cluster_nodes
            WHERE node_id = $1
            "#,
            id.0,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e: sqlx::Error| ClusterRepositoryError::Unknown(anyhow::anyhow!(e)))?;
        Ok(record.try_into()?)
    }
    async fn update_cluster_node_status(
        &self,
        req: &UpdateNodeStatusRequest,
    ) -> Result<ClusterNode, ClusterRepositoryError> {
        let record = sqlx::query_as!(
            ClusterNodeRecord,
            r#"
            INSERT INTO cluster_nodes (node_id, cluster_id, node_status, heartbeat_timestamp, memory_mb, cpu, gpu, reported_job_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT (node_id) DO UPDATE SET
                    node_status = EXCLUDED.node_status,
                    heartbeat_timestamp = EXCLUDED.heartbeat_timestamp,
                    reported_job_id = EXCLUDED.reported_job_id,
                    updated_at = NOW()
                RETURNING node_id, cluster_id, node_status as "node_status: NodeStatusRecord", heartbeat_timestamp, memory_mb, cpu as "cpu: CpuConfiguration", gpu as "gpu: GpuConfiguration", created_at, updated_at, assigned_job_id, reported_job_id;
            "#,
            req.node_id.0,
            req.cluster_id.0,
            NodeStatusRecord::from(req.status.clone()) as _,
            req.heartbeat_timestamp,
            req.memory_info,
            CpuConfiguration::from(req.cpu_info.clone()) as _,
            req.gpu_info.clone().map(|v| GpuConfiguration::from(v)) as _,
            req.job_info
                .as_ref()
                .and_then(|info| info.current_job_id)
                .map(|id| id.0),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e: sqlx::Error| ClusterRepositoryError::Unknown(anyhow::anyhow!(e)))?;
        Ok(record.try_into()?)
    }
    async fn delete_cluster_node(&self, node_id: &NodeId) -> Result<(), ClusterRepositoryError> {
        sqlx::query!("DELETE FROM cluster_nodes WHERE node_id = $1", node_id.0)
            .execute(&self.pool)
            .await
            .map_err(|e: sqlx::Error| ClusterRepositoryError::Unknown(anyhow::anyhow!(e)))?;
        Ok(())
    }

}

#[async_trait]
impl ClusterApiKeyRepository for PostgresClusterRepository {
    async fn create_api_key(&self, key: &ApiKey) -> Result<(), ClusterApiKeyRepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO api_keys (id, user_id, cluster_id, prefix, key_hash, created_at, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            key.id.inner(),
            key.user_id.map(|v| v.into_inner()),
            key.cluster_id.map(|v| v.into_inner()),
            key.prefix,
            key.key_hash,
            key.created_at,
            key.expires_at,
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|err| ClusterApiKeyRepositoryError::Unknown(anyhow::anyhow!(err)))
    }

    async fn find_cluster_by_api_key_hash(
        &self,
        key_hash: &str,
    ) -> Result<Cluster, ClusterApiKeyRepositoryError> {
        let record = sqlx::query_as!(
            ClusterRecord,
            r#"
            SELECT c.cluster_id, c.cluster_name, c.cluster_description, c.created_at, c.updated_at
            FROM clusters c
            JOIN api_keys ak ON c.cluster_id = ak.cluster_id
            WHERE ak.key_hash = $1 AND (ak.expires_at IS NULL OR ak.expires_at > now())
            "#,
            key_hash
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => ClusterApiKeyRepositoryError::NotFound,
            _ => ClusterApiKeyRepositoryError::Unknown(anyhow::anyhow!(err)),
        })?;

        Ok(record.into())
    }

    async fn list_api_keys_for_cluster(
        &self,
        cluster_id: &ClusterId,
    ) -> Result<Vec<ApiKey>, ClusterApiKeyRepositoryError> {
        let records = sqlx::query_as!(
            ApiKeyRecord,
            r#"
            SELECT id, user_id, cluster_id, prefix, key_hash, created_at, last_used_at, expires_at
            FROM api_keys
            WHERE cluster_id = $1
            "#,
            cluster_id.0
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|err| ClusterApiKeyRepositoryError::Unknown(anyhow::anyhow!(err)))?;

        Ok(records.into_iter().map(ApiKey::from).collect())
    }

    async fn delete_api_key(&self, id: &ApiKeyId) -> Result<(), ClusterApiKeyRepositoryError> {
        let result = sqlx::query!("DELETE FROM api_keys WHERE id = $1", id.inner())
            .execute(&self.pool)
            .await
            .map_err(|err| ClusterApiKeyRepositoryError::Unknown(anyhow::anyhow!(err)))?;

        if result.rows_affected() == 0 {
            return Err(ClusterApiKeyRepositoryError::NotFound);
        }

        Ok(())
    }
}
