use axum::{
    routing::{get, post},
    Router,
};

mod pipeline;
use pipeline::*;

pub fn router() -> Router {
    Router::new()
        .route("/pipeline", post(create_pipeline))
        .route(
            "/pipeline/{pipeline_id}",
            get(get_pipeline).delete(delete_pipeline),
        )
        .route("/pipeline/{pipeline_id}/run", get(run_pipeline))
}
