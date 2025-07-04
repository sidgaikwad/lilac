use axum::{
    extract::{Path, State},
    Json,
};
use common::{
    database::Database,
    k8s::K8sWrapper,
    model::{
        organization::{Organization, OrganizationId},
        roles::Role,
    },
    ServiceError,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use validator::Validate;

use crate::{auth::claims::Claims, tenants::register_tenant};

#[instrument(level = "info", skip(db), ret, err)]
pub async fn create_organization(
    claims: Claims,
    State(db): State<Database>,
    State(k8s): State<K8sWrapper>,
    Json(request): Json<CreateOrganizationRequest>,
) -> Result<Json<CreateOrganizationResponse>, ServiceError> {
    match request.validate() {
        Ok(_) => (),
        Err(e) => return Err(ServiceError::SchemaValidationError(e.to_string())),
    }
    let organization = Organization::create(request.organization_name);
    let org_id = organization.organization_id.clone();

    let owner = db.get_user(&claims.sub).await?;
    register_tenant(db, k8s, organization, owner).await?;

    Ok(Json(CreateOrganizationResponse { organization_id: org_id }))
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrganizationRequest {
    #[validate(length(min = 1, message = "Organization name cannot be empty"))]
    organization_name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateOrganizationResponse {
    organization_id: OrganizationId,
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn get_organization(
    _claims: Claims,
    State(db): State<Database>,
    Path(organization_id): Path<String>,
) -> Result<Json<GetOrganizationResponse>, ServiceError> {
    let organization_id = OrganizationId::try_from(organization_id)?;
    let organization = db.get_organization(&organization_id).await?.into();
    Ok(Json(organization))
}

#[derive(Debug, Serialize)]
pub struct GetOrganizationResponse {
    organization_id: OrganizationId,
    organization_name: String,
}

impl From<Organization> for GetOrganizationResponse {
    fn from(organization: Organization) -> Self {
        GetOrganizationResponse {
            organization_id: organization.organization_id,
            organization_name: organization.organization_name,
        }
    }
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn list_organizations(
    claims: Claims,
    State(db): State<Database>,
) -> Result<Json<ListOrganizationsResponse>, ServiceError> {
    let organizations = db
        .list_organizations(&claims.sub)
        .await?
        .into_iter()
        .map(|org| org.into())
        .collect();

    Ok(Json(ListOrganizationsResponse { organizations }))
}

#[derive(Debug, Serialize)]
pub struct ListOrganizationsResponse {
    organizations: Vec<GetOrganizationResponse>,
}
#[instrument(level = "info", skip(db), ret, err)]
pub async fn delete_organization(
    claims: Claims,
    State(db): State<Database>,
    Path(organization_id_str): Path<String>,
) -> Result<(), ServiceError> {
    let organization_id = OrganizationId::try_from(organization_id_str)?;

    let role = db.get_user_role(&claims.sub, &organization_id).await?;
    match role {
        Role::Owner => (),
        Role::Admin => return Err(ServiceError::Unauthorized),
        Role::Member => return Err(ServiceError::Unauthorized),
    }

    db.delete_organization(&organization_id).await?;

    Ok(())
}
