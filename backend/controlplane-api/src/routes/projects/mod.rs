use axum::{
    routing::{get, post},
    Router,
};

mod projects;
use projects::*;

pub fn router() -> Router {
    Router::new()
        .route("/projects", post(create_project))
        .route(
            "/projects/{project_id}",
            get(get_project).delete(delete_project),
        )
        .route("/projects/{project_id}/pipelines", get(list_project_pipelines))
}
