use axum::{routing::get, Router};

use crate::AppState;

mod organization;
use organization::{
    create_organization, delete_organization, get_organization, list_organizations,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/organizations",
            get(list_organizations).post(create_organization),
        )
        .route(
            "/organizations/{organization_id}",
            get(get_organization).delete(delete_organization),
        )
}
