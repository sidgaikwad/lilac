use axum::Router;

mod auth;
mod datasets;
mod job_outputs;
mod organization;
mod pipelines;
mod projects;
mod step_definitions;
mod steps;
mod user;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(pipelines::router())
        .merge(steps::router())
        .merge(projects::router())

        .merge(user::router())
        .merge(auth::router())
        .merge(organization::router())
        .merge(step_definitions::router())

        .merge(datasets::router())
}
