use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::inbound::http::AppState;

use self::handlers::{
    cancel_training_job, create_training_job, get_training_jobs, post_logs,
    update_training_job_status,
};

pub mod handlers;
pub mod models;

pub fn training_jobs_router() -> Router<AppState> {
    Router::new()
        .route("/training_jobs", post(create_training_job))
        .route("/training_jobs", get(get_training_jobs))
        .route(
            "/training_jobs/{job_id}/status",
            patch(update_training_job_status),
        )
        .route("/training_jobs/{job_id}/logs", post(post_logs))
        .route(
            "/training_jobs/{job_id}/cancel",
            post(cancel_training_job),
        )
}
