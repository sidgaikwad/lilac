use axum::{routing::{get, delete}, Router};
mod organization;
use organization::{create_organization, get_organization, list_organizations, delete_organization_handler};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/organizations", get(list_organizations).post(create_organization))
        .route("/organizations/{organization_id}", get(get_organization).delete(delete_organization_handler))
}