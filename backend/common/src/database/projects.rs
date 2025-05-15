use crate::{
    model::{
        organization::OrganizationId,
        project::{Project, ProjectId},
        user::UserId,
    },
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

    pub async fn list_projects_for_user(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<Project>, ServiceError> {
        let id = user_id.inner();
        let orgs = sqlx::query_as!(
            Project,
            // language=PostgreSQL
            r#"
                SELECT project_id, project_name, organization_id FROM "projects" WHERE organization_id = ANY(SELECT organization_id FROM "organization_memberships" WHERE user_id = $1)
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(orgs)
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
        let mut tx = self.pool.begin().await?;
        let project_id_inner = project_id.inner();

        // Step 1: Check for child Pipelines
        let pipeline_exists = sqlx::query!(
            // language=PostgreSQL
            r#"
                SELECT EXISTS (SELECT 1 FROM "pipelines" WHERE project_id = $1 LIMIT 1) AS "exists!"
            "#,
            project_id_inner
        )
        .fetch_one(&mut *tx)
        .await?
        .exists;

        if pipeline_exists {
            return Err(ServiceError::Conflict(
                "Project cannot be deleted as it still contains pipelines. Please delete them first.".to_string(),
            ));
        }

        // Step 2: Check for child Datasets
        let dataset_exists = sqlx::query!(
            // language=PostgreSQL
            r#"
                SELECT EXISTS (SELECT 1 FROM "datasets" WHERE project_id = $1 LIMIT 1) AS "exists!"
            "#,
            project_id_inner
        )
        .fetch_one(&mut *tx)
        .await?
        .exists;

        if dataset_exists {
            return Err(ServiceError::Conflict(
                "Project cannot be deleted as it still contains datasets. Please delete them first.".to_string(),
            ));
        }

        // Step 3: Delete Project
        sqlx::query!(
            // language=PostgreSQL
            r#"
                DELETE FROM "projects" WHERE project_id = $1
            "#,
            project_id_inner
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }
}
