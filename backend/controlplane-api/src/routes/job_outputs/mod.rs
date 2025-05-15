use axum::{routing::get, Router};
mod job_outputs;
use job_outputs::{list_job_output_images_handler, list_job_outputs_handler};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_job_outputs_handler))
        .route("/{job_id}/images", get(list_job_output_images_handler))
}