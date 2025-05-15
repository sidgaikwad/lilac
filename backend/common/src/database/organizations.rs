use crate::{
    model::{
        organization::{Organization, OrganizationId},
        user::UserId,
    },
    ServiceError,
};

use super::Database;

impl Database {
    pub async fn get_organization(
        &self,
        organization_id: &OrganizationId,
    ) -> Result<Organization, ServiceError> {
        let id = organization_id.inner();
        let organization = sqlx::query_as!(
            Organization,
            // language=PostgreSQL
            r#"
                SELECT organization_id, organization_name FROM "organizations" WHERE organization_id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(organization)
    }

    pub async fn list_organizations(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<Organization>, ServiceError> {
        let id = user_id.inner();
        let orgs = sqlx::query_as!(
            Organization,
            // language=PostgreSQL
            r#"
                SELECT o.organization_id, o.organization_name FROM "organization_memberships" m INNER JOIN organizations o ON m.organization_id = o.organization_id WHERE m.user_id = $1
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(orgs)
    }

    pub async fn create_organization(
        &self,
        organization: Organization,
    ) -> Result<OrganizationId, ServiceError> {
        let org_id = sqlx::query!(
            // language=PostgreSQL
            r#"
                INSERT INTO "organizations" (organization_id, organization_name) VALUES ($1, $2) RETURNING organization_id
            "#,
            organization.organization_id.inner(),
            &organization.organization_name
        )
        .map(|row| OrganizationId::new(row.organization_id))
        .fetch_one(&self.pool)
        .await?;
        Ok(org_id)
    }

    pub async fn join_organization(
        &self,
        organization_id: &OrganizationId,
        user_id: &UserId,
    ) -> Result<(), ServiceError> {
        let _ = sqlx::query!(
            // language=PostgreSQL
            r#"
                INSERT INTO "organization_memberships" (organization_id, user_id) VALUES ($1, $2)
            "#,
            organization_id.inner(),
            user_id.inner(),
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    pub async fn is_user_member_of_organization(
        &self,
        user_id: &UserId,
        organization_id: &OrganizationId,
    ) -> Result<bool, ServiceError> {
        let result = sqlx::query!(
            // language=PostgreSQL
            r#"
                SELECT EXISTS (SELECT 1 FROM "organization_memberships" WHERE user_id = $1 AND organization_id = $2) AS "exists!"
            "#,
            user_id.inner(),
            organization_id.inner()
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result.exists)
    }

    pub async fn delete_organization(&self, organization_id: &OrganizationId) -> Result<(), ServiceError> {
        let mut tx = self.pool.begin().await?;
        let org_id_inner = organization_id.inner();

        // Step 1: Check for child Projects
        let project_exists = sqlx::query!(
            // language=PostgreSQL
            r#"
                SELECT EXISTS (SELECT 1 FROM "projects" WHERE organization_id = $1 LIMIT 1) AS "exists!"
            "#,
            org_id_inner
        )
        .fetch_one(&mut *tx)
        .await?
        .exists;

        if project_exists {
            return Err(ServiceError::Conflict(
                "Organization cannot be deleted as it still contains projects. Please delete them first.".to_string(),
            ));
        }

        // Step 2: Delete Memberships
        sqlx::query!(
            // language=PostgreSQL
            r#"
                DELETE FROM "organization_memberships" WHERE organization_id = $1
            "#,
            org_id_inner
        )
        .execute(&mut *tx)
        .await?;

        // Step 3: Delete Organization
        sqlx::query!(
            // language=PostgreSQL
            r#"
                DELETE FROM "organizations" WHERE organization_id = $1
            "#,
            org_id_inner
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }
}
