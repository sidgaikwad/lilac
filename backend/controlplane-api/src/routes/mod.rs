use axum::{
    routing::{get, post},
    Router,
};

mod user;
pub use user::*;

mod auth;
pub use auth::*;

mod organization;
pub use organization::*;

mod step_definitions;
pub use step_definitions::*;

mod pipelines;
mod projects;
mod steps;

pub fn router() -> Router {
    Router::new()
        .merge(pipelines::router())
        .merge(steps::router())
        .merge(projects::router())
        // user routes
        .route("/account/details", get(user::get_current_user))
        .route("/users", post(user::create_user))
        .route("/users/{user_id}", get(user::get_user))
        // auth routes
        .route("/auth/login", post(auth::authorize))
        // organizatino routes
        .route(
            "/organizations",
            get(organization::list_organizations).post(organization::create_organization),
        )
        .route(
            "/organizations/{organization_id}",
            get(organization::get_organization),
        )
        // step definitions routes
        .route(
            "/step_definitions",
            get(step_definitions::list_step_definitions),
        )
}
