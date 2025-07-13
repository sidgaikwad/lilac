use crate::{
    database::{translate_sqlx_error, DatabaseError},
    model::{
        project::{Project, ProjectId},
        user::UserId,
    },
};

use super::Database;

impl Database {
    pub async fn create_project_with_membership(
        &self,
        project: Project,
        user_id: &UserId,
    ) -> Result<(), DatabaseError> {
        let mut tx = self.pool.begin().await?;

        let project_id = project.project_id.clone();

        sqlx::query!(
            // language=PostgreSQL
            r#"
                INSERT INTO "projects" (project_id, project_name) VALUES ($1, $2)
            "#,
            project.project_id.inner(),
            &project.project_name
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| translate_sqlx_error(String::from("project"), project_id.to_string(), e))?;

        sqlx::query!(
            // language=PostgreSQL
            r#"
                INSERT INTO "project_memberships" (project_id, user_id, role) VALUES ($1, $2, $3)
            "#,
            project_id.inner(),
            user_id.inner(),
            "owner"
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn list_projects_for_user(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<Project>, DatabaseError> {
        let id = user_id.inner();
        #[derive(sqlx::FromRow)]
        struct ProjectRow {
            project_id: uuid::Uuid,
            project_name: String,
            aws_integration: Option<serde_json::Value>,
        }

        let rows = sqlx::query_as!(
            ProjectRow,
            // language=PostgreSQL
            r#"
                SELECT p.project_id, p.project_name, p.aws_integration
                FROM "project_memberships" m
                INNER JOIN projects p ON m.project_id = p.project_id
                WHERE m.user_id = $1
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await?;

        let projects = rows
            .into_iter()
            .map(|row| Project {
                project_id: row.project_id.into(),
                project_name: row.project_name,
                aws_integration: row
                    .aws_integration
                    .and_then(|v| serde_json::from_value(v).ok()),
            })
            .collect();

        Ok(projects)
    }

    pub async fn get_project(&self, project_id: &ProjectId) -> Result<Project, DatabaseError> {
        #[derive(sqlx::FromRow)]
        struct ProjectRow {
            project_id: uuid::Uuid,
            project_name: String,
            aws_integration: Option<serde_json::Value>,
        }

        let id = project_id.inner();
        let row = sqlx::query_as!(
            ProjectRow,
            // language=PostgreSQL
            r#"
                SELECT project_id, project_name, aws_integration
                FROM "projects"
                WHERE project_id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        let project = Project {
            project_id: row.project_id.into(),
            project_name: row.project_name,
            aws_integration: row
                .aws_integration
                .and_then(|v| serde_json::from_value(v).ok()),
        };

        Ok(project)
    }

    pub async fn delete_project(&self, project_id: &ProjectId) -> Result<(), DatabaseError> {
        let mut tx = self.pool.begin().await?;
        let id = project_id.inner();

        sqlx::query!(
            // language=PostgreSQL
            r#"
                DELETE FROM "project_memberships" WHERE project_id = $1
            "#,
            id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            // language=PostgreSQL
            r#"
                DELETE FROM "projects" WHERE project_id = $1
            "#,
            id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn is_user_project_member(
        &self,
        user_id: &UserId,
        project_id: &ProjectId,
    ) -> Result<bool, DatabaseError> {
        let user_id = user_id.inner();
        let project_id = project_id.inner();

        let count = sqlx::query!(
            // language=PostgreSQL
            r#"
                SELECT count(*) as "count!" FROM "project_memberships" WHERE user_id = $1 AND project_id = $2
            "#,
            user_id,
            project_id
        )
        .fetch_one(&self.pool)
        .await?
        .count;

        Ok(count > 0)
    }
}
