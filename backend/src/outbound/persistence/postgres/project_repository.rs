use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::{
    project::{
        models::{CreateProjectRequest, Project, ProjectId},
        ports::{ProjectRepository, ProjectRepositoryError},
    },
    user::models::UserId,
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

#[async_trait]
impl ProjectRepository for PostgresProjectRepository {
    async fn create_project(
        &self,
        req: &CreateProjectRequest,
    ) -> Result<Project, ProjectRepositoryError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| ProjectRepositoryError::Unknown(e.into()))?;

        let project_id = ProjectId(uuid::Uuid::new_v4());

        sqlx::query!(
            "INSERT INTO projects (project_id, project_name, owner_id) VALUES ($1, $2, $3)",
            project_id.0,
            req.name,
            req.owner_id.unwrap().0,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.is_unique_violation() {
                    return ProjectRepositoryError::Duplicate("name".to_string(), req.name.clone());
                }
            }
            ProjectRepositoryError::Unknown(e.into())
        })?;

        sqlx::query!(
            "INSERT INTO project_memberships (project_id, user_id, role) VALUES ($1, $2, 'admin')",
            project_id.0,
            req.owner_id.unwrap().0
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ProjectRepositoryError::Unknown(e.into()))?;

        tx.commit()
            .await
            .map_err(|e| ProjectRepositoryError::Unknown(e.into()))?;

        self.get_project_by_id(&project_id).await
    }

    async fn get_project_by_id(&self, id: &ProjectId) -> Result<Project, ProjectRepositoryError> {
        sqlx::query_as!(
            Project,
            r#"
            SELECT project_id as "id: _", project_name as "name", created_at, updated_at, owner_id as "owner_id:_"
            FROM projects
            WHERE project_id = $1
            "#,
            id.0
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ProjectRepositoryError::NotFound(id.0.to_string()),
            _ => ProjectRepositoryError::Unknown(anyhow::anyhow!(e)),
        })
    }

    async fn list_projects_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<Project>, ProjectRepositoryError> {
        sqlx::query_as!(
            Project,
            r#"
            SELECT p.project_id as "id: _", p.project_name as "name", p.created_at, p.updated_at, p.owner_id as "owner_id:_"
            FROM projects p
            JOIN project_memberships pm ON p.project_id = pm.project_id
            WHERE pm.user_id = $1
            "#,
            user_id.0
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e: sqlx::Error| ProjectRepositoryError::Unknown(anyhow::anyhow!(e)))
    }

    async fn delete_project(&self, id: &ProjectId) -> Result<(), ProjectRepositoryError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| ProjectRepositoryError::Unknown(e.into()))?;

        sqlx::query!(
            "DELETE FROM project_memberships WHERE project_id = $1",
            id.0
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ProjectRepositoryError::Unknown(e.into()))?;

        sqlx::query!("DELETE FROM projects WHERE project_id = $1", id.0)
            .execute(&mut *tx)
            .await
            .map(|_| ())
            .map_err(|e: sqlx::Error| ProjectRepositoryError::Unknown(anyhow::anyhow!(e)))?;

        tx.commit()
            .await
            .map_err(|e| ProjectRepositoryError::Unknown(e.into()))
    }

    async fn is_user_project_member(
        &self,
        user_id: &UserId,
        project_id: &ProjectId,
    ) -> Result<bool, ProjectRepositoryError> {
        sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM project_memberships WHERE user_id = $1 AND project_id = $2)",
            user_id.0,
            project_id.0
        )
        .fetch_one(&self.pool)
        .await
        .map(|opt| opt.unwrap_or(false))
        .map_err(|e: sqlx::Error| ProjectRepositoryError::Unknown(anyhow::anyhow!(e)))
    }

    async fn add_user_to_project(
        &self,
        project_id: &ProjectId,
        user_id: &UserId,
        role: &str,
    ) -> Result<(), ProjectRepositoryError> {
        sqlx::query!(
            "INSERT INTO project_memberships (project_id, user_id, role) VALUES ($1, $2, $3)",
            project_id.0,
            user_id.0,
            role
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|e: sqlx::Error| ProjectRepositoryError::Unknown(anyhow::anyhow!(e)))
    }
}
