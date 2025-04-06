use axum::{extract::Path, Extension, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::instrument;

use crate::{
    auth::claims::Claims, database, model::organization::{Organization, OrganizationId}, ServiceError
};

#[instrument(skip(db))]
pub async fn create_organization(
    claims: Claims,
    db: Extension<PgPool>,
    Json(request): Json<CreateOrganizationRequest>,
) -> Result<Json<CreateOrganizationResponse>, ServiceError> {
    let organization = Organization::create(request.organization_name);

    let org_id = database::create_organization(&db, organization).await?;

    let user_id = claims.sub;
    database::join_organization(&db, &org_id, &user_id).await?;

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

#[instrument(skip(db))]
pub async fn get_organization(
    _claims: Claims,
    db: Extension<PgPool>,
    Path(organization_id): Path<String>,
) -> Result<Json<GetOrganizationResponse>, ServiceError> {
    let organization_id = OrganizationId::try_from(organization_id)?;
    let organization = database::get_organization(&db, &organization_id)
        .await?
        .into();
    Ok(Json(organization))
}

#[derive(Debug, Serialize)]
pub struct GetOrganizationResponse {
    id: OrganizationId,
    organization_name: String,
    created_at: DateTime<Utc>,
}

impl From<Organization> for GetOrganizationResponse {
    fn from(organization: Organization) -> Self {
        GetOrganizationResponse {
            id: organization.organization_id,
            organization_name: organization.organization_name,
            created_at: organization.created_at,
        }
    }
}

#[instrument(skip(db))]
pub async fn list_organizations(
    claims: Claims,
    db: Extension<PgPool>,
) -> Result<Json<ListOrganizationsResponse>, ServiceError> {
    let organizations = database::list_organizations(&db, &claims.sub)
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
