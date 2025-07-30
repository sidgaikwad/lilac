use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    // cluster::models::ClusterId,
    project::models::ProjectId,
    user::models::UserId,
    workspace::{
        models::{CreateWorkspaceRequest, Ide, Workspace, WorkspaceId, WorkspaceStatus},
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

#[derive(sqlx::Type)]
#[sqlx(type_name = "ide_type", rename_all = "lowercase")]
enum IdeRecord {
    Vscode,
    JupyterLab,
    RStudio,
}

impl From<Ide> for IdeRecord {
    fn from(value: Ide) -> Self {
        match value {
            Ide::Vscode => Self::Vscode,
            Ide::JupyterLab => Self::JupyterLab,
            Ide::RStudio => Self::RStudio,
        }
    }
}

impl From<IdeRecord> for Ide {
    fn from(value: IdeRecord) -> Self {
        match value {
            IdeRecord::Vscode => Self::Vscode,
            IdeRecord::JupyterLab => Self::JupyterLab,
            IdeRecord::RStudio => Self::RStudio,
        }
    }
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "workspace_status", rename_all = "lowercase")]
enum WorkspaceStatusRecord {
    Pending,
    Running,
    Stopping,
    Stopped,
    Failed,
    Terminated,
}

impl From<WorkspaceStatus> for WorkspaceStatusRecord {
    fn from(value: WorkspaceStatus) -> Self {
        match value {
            WorkspaceStatus::Pending => Self::Pending,
            WorkspaceStatus::Running => Self::Running,
            WorkspaceStatus::Stopping => Self::Stopping,
            WorkspaceStatus::Stopped => Self::Stopped,
            WorkspaceStatus::Failed => Self::Failed,
            WorkspaceStatus::Terminated => Self::Terminated,
        }
    }
}

impl From<WorkspaceStatusRecord> for WorkspaceStatus {
    fn from(value: WorkspaceStatusRecord) -> Self {
        match value {
            WorkspaceStatusRecord::Pending => Self::Pending,
            WorkspaceStatusRecord::Running => Self::Running,
            WorkspaceStatusRecord::Stopping => Self::Stopping,
            WorkspaceStatusRecord::Stopped => Self::Stopped,
            WorkspaceStatusRecord::Failed => Self::Failed,
            WorkspaceStatusRecord::Terminated => Self::Terminated,
        }
    }
}

#[derive(sqlx::FromRow)]
struct WorkspaceRecord {
    workspace_id: Uuid,
    workspace_name: String,
    project_id: Uuid,
    owner_id: Uuid,
    cluster_id: Uuid,
    ide: IdeRecord,
    image: String,
    cpu_millicores: i32,
    memory_mb: i32,
    gpu: bool,
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
            ide: record.ide.into(),
            image: record.image,
            cpu_millicores: record.cpu_millicores,
            memory_mb: record.memory_mb,
            gpu: record.gpu,
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
            INSERT INTO workspaces (workspace_name, project_id, owner_id, cluster_id, ide, image, cpu_millicores, memory_mb, gpu)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING
                workspace_id,
                workspace_name,
                project_id,
                owner_id,
                cluster_id,
                ide as "ide: IdeRecord",
                image,
                cpu_millicores,
                memory_mb,
                gpu,
                status as "status: WorkspaceStatusRecord",
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
            IdeRecord::from(req.ide.clone()) as _,
            req.image,
            req.cpu_millicores,
            req.memory_mb,
            req.gpu
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
                ide as "ide: IdeRecord",
                image,
                cpu_millicores,
                memory_mb,
                gpu,
                status as "status: WorkspaceStatusRecord",
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
        status: WorkspaceStatus,
        url: &str,
    ) -> Result<(), WorkspaceRepositoryError> {
        sqlx::query!(
            r#"
            UPDATE workspaces
            SET status = $1, url = $2, updated_at = NOW()
            WHERE workspace_id = $3
            "#,
            WorkspaceStatusRecord::from(status) as _,
            url,
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
                ide as "ide: IdeRecord",
                image,
                cpu_millicores,
                memory_mb,
                gpu,
                status as "status: WorkspaceStatusRecord",
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
