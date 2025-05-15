use axum::{routing::get, Router};
mod organization;
use organization::{create_organization, get_organization, list_organizations};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/organizations", get(list_organizations).post(create_organization))
        .route("/organizations/{organization_id}", get(get_organization))
}