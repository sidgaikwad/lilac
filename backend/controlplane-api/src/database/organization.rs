use sqlx::PgPool;

use crate::{
    model::{
        organization::{Organization, OrganizationId},
        user::UserId,
    },
    ServiceError,
};

pub async fn get_organization(
    db: &PgPool,
    organization_id: &OrganizationId,
) -> Result<Organization, ServiceError> {
    let id = organization_id.inner();
    let organization = sqlx::query_as!(
        Organization,
        // language=PostgreSQL
        r#"
            SELECT * FROM "organizations" WHERE organization_id = $1
        "#,
        id
    )
    .fetch_one(db)
    .await?;
    Ok(organization)
}

pub async fn create_organization(
    db: &PgPool,
    organization: Organization,
) -> Result<OrganizationId, ServiceError> {
    let org_id = sqlx::query!(
        // language=PostgreSQL
        r#"
            INSERT INTO "organizations" (organization_id, organization_name, created_at) VALUES ($1, $2, $3) RETURNING organization_id
        "#,
        organization.organization_id.inner(),
        &organization.organization_name,
        &organization.created_at,
    )
    .map(|row| OrganizationId::new(row.organization_id))
    .fetch_one(db)
    .await?;
    Ok(org_id)
}

pub async fn join_organization(
    db: &PgPool,
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
    .execute(db)
    .await?;
    Ok(())
}
