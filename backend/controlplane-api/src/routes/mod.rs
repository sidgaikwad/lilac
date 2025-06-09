use axum::Router;

mod auth;
mod datasets;
mod organization;
mod projects;
mod user;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(projects::router())
        .merge(user::router())
        .merge(auth::router())
        .merge(organization::router())
        .merge(datasets::router())
}
