use axum::{extract::{Path, State}, Json};
use common::{
    database::Database,
    model::organization::{Organization, OrganizationId},
    ServiceError,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use validator::Validate;

use crate::auth::claims::Claims;

#[instrument(level = "info", skip(db), ret, err)]
pub async fn create_organization(
    claims: Claims,
    State(db): State<Database>,
    Json(request): Json<CreateOrganizationRequest>,
) -> Result<Json<CreateOrganizationResponse>, ServiceError> {
    match request.validate() {
        Ok(_) => (),
        Err(e) => return Err(ServiceError::SchemaValidationError(e.to_string())),
    }
    let organization = Organization::create(request.name);

    let org_id = db.create_organization(organization).await?;

    let user_id = claims.sub;
    db.join_organization(&org_id, &user_id).await?;

    Ok(Json(CreateOrganizationResponse { id: org_id }))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrganizationRequest {
    #[validate(length(min = 1, message = "Organization name cannot be empty"))]
    name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrganizationResponse {
    id: OrganizationId,
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
#[serde(rename_all = "camelCase")]
pub struct GetOrganizationResponse {
    id: OrganizationId,
    name: String,
}

impl From<Organization> for GetOrganizationResponse {
    fn from(organization: Organization) -> Self {
        GetOrganizationResponse {
            id: organization.organization_id,
            name: organization.organization_name,
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
#[serde(rename_all = "camelCase")]
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

    let is_member = db
        .is_user_member_of_organization(&claims.sub, &organization_id)
        .await?;
    if !is_member {
        return Err(ServiceError::Unauthorized);
    }

    db.delete_organization(&organization_id).await?;

    Ok(())
}