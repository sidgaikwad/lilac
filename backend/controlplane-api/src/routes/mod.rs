use axum::Router;

mod auth;
mod datasets;
mod projects;
mod services;
mod user;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(projects::router())
        .merge(user::router())
        .merge(auth::router())
        .merge(datasets::router())
        .merge(services::router())
}
