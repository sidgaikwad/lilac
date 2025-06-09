use axum::{routing::get, Router};

use crate::AppState;

mod organization;
use organization::{create_organization, get_organization, list_organizations, delete_organization};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/organizations", get(list_organizations).post(create_organization))
        .route("/organizations/{organization_id}", get(get_organization).delete(delete_organization))
}