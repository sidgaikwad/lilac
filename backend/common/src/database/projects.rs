use crate::{
    model::{organization::OrganizationId, project::{Project, ProjectId}},
    ServiceError,
};

use super::Database;

impl Database {
    pub async fn get_project(&self, project_id: &ProjectId) -> Result<Project, ServiceError> {
        let id = project_id.inner();
        let project = sqlx::query_as!(
            Project,
            // language=PostgreSQL
            r#"
                SELECT project_id, project_name, organization_id FROM "projects" WHERE project_id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(project)
    }

    pub async fn list_projects(
        &self,
        organization_id: &OrganizationId,
    ) -> Result<Vec<Project>, ServiceError> {
        let id = organization_id.inner();
        let orgs = sqlx::query_as!(
            Project,
            // language=PostgreSQL
            r#"
                SELECT project_id, project_name, organization_id FROM "projects" WHERE organization_id = $1
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(orgs)
    }

    pub async fn create_project(&self, project: Project) -> Result<ProjectId, ServiceError> {
        let org_id = sqlx::query!(
            // language=PostgreSQL
            r#"
                INSERT INTO "projects" (project_id, project_name, organization_id) VALUES ($1, $2, $3) RETURNING project_id
            "#,
            project.project_id.inner(),
            &project.project_name,
            project.organization_id.inner()
        )
        .map(|row| ProjectId::new(row.project_id))
        .fetch_one(&self.pool)
        .await?;
        Ok(org_id)
    }

    pub async fn delete_project(&self, project_id: &ProjectId) -> Result<(), ServiceError> {
        sqlx::query!(
            // language=PostgreSQL
            r#"
                DELETE FROM "projects" WHERE project_id = $1
            "#,
            project_id.inner()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
