use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::domain::project::{
    models::{CreateProjectRequest, Project, ProjectId},
    ports::{ProjectRepository, ProjectRepositoryError},
};

#[derive(Clone)]
pub struct PostgresProjectRepository {
    pool: PgPool,
}

impl PostgresProjectRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct ProjectRecord {
    project_id: uuid::Uuid,
    project_name: String,
    owner_id: uuid::Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<ProjectRecord> for Project {
    fn from(record: ProjectRecord) -> Self {
        Self {
            id: record.project_id.into(),
            name: record.project_name,
            owner_id: record.owner_id.into(),
            created_at: record.created_at,
            updated_at: record.updated_at,
        }
    }
}

#[async_trait]
impl ProjectRepository for PostgresProjectRepository {
    async fn create_project(
        &self,
        req: &CreateProjectRequest,
    ) -> Result<Project, ProjectRepositoryError> {
        let project_id = ProjectId::generate();

        let record = sqlx::query_as!(
            ProjectRecord,
            "INSERT INTO projects (project_id, project_name, owner_id) VALUES ($1, $2, $3) RETURNING project_id, project_name, owner_id, created_at, updated_at",
            project_id.inner(),
            req.name,
            req.owner_id.inner(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            ProjectRepositoryError::Unknown(e.into())
        })?;
        Ok(record.into())
    }

    async fn get_project_by_id(&self, id: &ProjectId) -> Result<Project, ProjectRepositoryError> {
        sqlx::query_as!(
            ProjectRecord,
            r#"
            SELECT project_id, project_name, created_at, updated_at, owner_id
            FROM projects
            WHERE project_id = $1
            "#,
            id.inner()
        )
        .fetch_one(&self.pool)
        .await
        .map(|record| record.into())
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                ProjectRepositoryError::NotFound(id.into_inner().to_string())
            }
            _ => ProjectRepositoryError::Unknown(anyhow::anyhow!(e)),
        })
    }

    async fn list_projects(&self) -> Result<Vec<Project>, ProjectRepositoryError> {
        sqlx::query_as!(
            ProjectRecord,
            r#"
            SELECT project_id, project_name, created_at, updated_at, owner_id
            FROM projects
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map(|records| records.into_iter().map(|record| record.into()).collect())
        .map_err(|e: sqlx::Error| ProjectRepositoryError::Unknown(anyhow::anyhow!(e)))
    }

    async fn delete_project(&self, id: &ProjectId) -> Result<(), ProjectRepositoryError> {
        sqlx::query!("DELETE FROM projects WHERE project_id = $1", id.inner())
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e: sqlx::Error| ProjectRepositoryError::Unknown(anyhow::anyhow!(e)))?;
        Ok(())
    }
}
