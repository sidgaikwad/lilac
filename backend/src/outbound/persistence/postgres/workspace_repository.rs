use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    // cluster::models::ClusterId,
    project::models::ProjectId,
    user::models::UserId,
    workspace::{
        models::{
            CreateWorkspaceRequest, Ide, Workspace, WorkspaceId,
            WorkspaceStatus,
        },
        ports::{WorkspaceRepository, WorkspaceRepositoryError},
    },
};

pub struct PostgresWorkspaceRepository {
    pool: PgPool,
}

impl PostgresWorkspaceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct WorkspaceRecord {
    workspace_id: Uuid,
    workspace_name: String,
    project_id: Uuid,
    owner_id: Uuid,
    cluster_id: Uuid,
    ide: Ide,
    image: String,
    cpu_millicores: i32,
    memory_mb: i32,
    status: WorkspaceStatus,
    url: Option<String>,
    token: Option<String>,
    public_key: Option<String>,
    private_key: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<WorkspaceRecord> for Workspace {
    fn from(record: WorkspaceRecord) -> Self {
        Self {
            id: record.workspace_id.into(),
            name: record.workspace_name,
            project_id: record.project_id.into(),
            owner_id: record.owner_id.into(),
            cluster_id: record.cluster_id.into(),
            ide: record.ide,
            image: record.image,
            cpu_millicores: record.cpu_millicores,
            memory_mb: record.memory_mb,
            status: record.status,
            url: record.url,
            token: record.token,
            public_key: record.public_key,
            private_key: record.private_key,
            created_at: record.created_at,
            updated_at: record.updated_at,
        }
    }
}

#[async_trait]
impl WorkspaceRepository for PostgresWorkspaceRepository {
    async fn create(
        &self,
        req: &CreateWorkspaceRequest,
        owner_id: UserId,
    ) -> Result<Workspace, WorkspaceRepositoryError> {
        let record = sqlx::query_as!(
            WorkspaceRecord,
            r#"
            INSERT INTO workspaces (workspace_name, project_id, owner_id, cluster_id, ide, image, cpu_millicores, memory_mb)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING
                workspace_id,
                workspace_name,
                project_id,
                owner_id,
                cluster_id,
                ide as "ide: Ide",
                image,
                cpu_millicores,
                memory_mb,
                status as "status: WorkspaceStatus",
                url,
                token,
                public_key,
                private_key,
                created_at,
                updated_at;
            "#,
            req.name,
            req.project_id.inner(),
            owner_id.inner(),
            req.cluster_id.inner(),
            req.ide as _,
            req.image,
            req.cpu_millicores,
            req.memory_mb
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| WorkspaceRepositoryError::Other(e.into()))?;

        Ok(record.into())
    }

    async fn find_by_id(&self, id: WorkspaceId) -> Result<Workspace, WorkspaceRepositoryError> {
        let record = sqlx::query_as!(
            WorkspaceRecord,
            r#"
            SELECT
                workspace_id,
                workspace_name,
                project_id,
                owner_id,
                cluster_id,
                ide as "ide: Ide",
                image,
                cpu_millicores,
                memory_mb,
                status as "status: WorkspaceStatus",
                url,
                token,
                public_key,
                private_key,
                created_at,
                updated_at
            FROM workspaces
            WHERE workspace_id = $1
            "#,
            id.inner()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => WorkspaceRepositoryError::NotFound,
            _ => WorkspaceRepositoryError::Other(e.into()),
        })?;

        Ok(record.into())
    }

    async fn update_connection_details(
        &self,
        id: WorkspaceId,
        status: crate::domain::workspace::models::WorkspaceStatus,
        url: &str,
        token: &str,
    ) -> Result<(), WorkspaceRepositoryError> {
        sqlx::query!(
            r#"
            UPDATE workspaces
            SET status = $1, url = $2, token = $3, updated_at = NOW()
            WHERE workspace_id = $4
            "#,
            status as _,
            url,
            token,
            id.inner()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| WorkspaceRepositoryError::Other(e.into()))?;

        Ok(())
    }

    async fn list_by_project_id(
        &self,
        project_id: ProjectId,
    ) -> Result<Vec<Workspace>, WorkspaceRepositoryError> {
        let records = sqlx::query_as!(
            WorkspaceRecord,
            r#"
            SELECT
                workspace_id,
                workspace_name,
                project_id,
                owner_id,
                cluster_id,
                ide as "ide: Ide",
                image,
                cpu_millicores,
                memory_mb,
                status as "status: WorkspaceStatus",
                url,
                token,
                public_key,
                private_key,
                created_at,
                updated_at
            FROM workspaces
            WHERE project_id = $1
            "#,
            project_id.inner()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| WorkspaceRepositoryError::Other(e.into()))?;

        Ok(records.into_iter().map(|r| r.into()).collect())
    }
}