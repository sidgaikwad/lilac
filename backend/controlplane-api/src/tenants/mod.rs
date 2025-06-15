use common::{
    database::Database,
    k8s::{K8sApi, K8sWrapper},
    model::{organization::Organization, roles::Role, user::User},
    ServiceError,
};

pub async fn register_tenant(
    db: Database,
    k8s: K8sWrapper,
    organization: Organization,
    owner: User,
) -> Result<(), ServiceError> {
    db.create_organization(organization.clone()).await?;
    db.join_organization(&organization.organization_id, &owner.user_id, Role::Owner)
        .await?;
    k8s.create_namespace(&organization.organization_id.to_string())
        .await?;
    Ok(())
}

pub async fn add_user_to_tenant(
    db: Database,
    organization: Organization,
    user: User,
    role: Role,
) -> Result<(), ServiceError> {
    db.join_organization(&organization.organization_id, &user.user_id, role)
        .await?;

    Ok(())
}
