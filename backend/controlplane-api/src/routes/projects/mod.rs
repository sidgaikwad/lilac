use axum::{routing::get, Router};

mod projects;
use projects::*;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/projects", get(list_projects).post(create_project))
        .route(
            "/projects/{project_id}",
            get(get_project).delete(delete_project),
        )
        .route(
            "/projects/{project_id}/pipelines",
            get(list_project_pipelines),
        )
}
