use axum::{extract::Path, Extension, Json};
use common::{
    database::Database,
    model::organization::{Organization, OrganizationId},
    ServiceError,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::auth::claims::Claims;

#[instrument(level = "info", skip(db), ret, err)]
pub async fn create_organization(
    claims: Claims,
    db: Extension<Database>,
    Json(request): Json<CreateOrganizationRequest>,
) -> Result<Json<CreateOrganizationResponse>, ServiceError> {
    let organization = Organization::create(request.organization_name);

    let org_id = db.create_organization(organization).await?;

    let user_id = claims.sub;
    db.join_organization(&org_id, &user_id).await?;

    Ok(Json(CreateOrganizationResponse { id: org_id }))
}

#[derive(Debug, Deserialize)]
pub struct CreateOrganizationRequest {
    organization_name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateOrganizationResponse {
    id: OrganizationId,
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn get_organization(
    _claims: Claims,
    db: Extension<Database>,
    Path(organization_id): Path<String>,
) -> Result<Json<GetOrganizationResponse>, ServiceError> {
    let organization_id = OrganizationId::try_from(organization_id)?;
    let organization = db.get_organization(&organization_id).await?.into();
    Ok(Json(organization))
}

#[derive(Debug, Serialize)]
pub struct GetOrganizationResponse {
    id: OrganizationId,
    organization_name: String,
}

impl From<Organization> for GetOrganizationResponse {
    fn from(organization: Organization) -> Self {
        GetOrganizationResponse {
            id: organization.organization_id,
            organization_name: organization.organization_name,
        }
    }
}

#[instrument(skip(db))]
pub async fn list_organizations(
    claims: Claims,
    db: Extension<Database>,
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
