use axum::{
    routing::{get, post},
    Router,
};

mod pipelines;
use pipelines::*;

pub fn router() -> Router {
    Router::new()
        .route("/pipelines", post(create_pipeline))
        .route(
            "/pipelines/{pipeline_id}",
            get(get_pipeline).delete(delete_pipeline),
        )
        .route("/pipelines/{pipeline_id}/run", post(run_pipeline))
}
