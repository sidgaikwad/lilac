use axum::{routing::{post, get, patch}, Router};

use crate::inbound::http::AppState;

use self::handlers::{create_training_job, get_training_jobs, schedule_training_job, update_training_job_status, post_logs};

pub mod handlers;
pub mod models;

pub fn training_jobs_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_training_job))
        .route("/", get(get_training_jobs))
        .route("/{:job_id}/schedule", post(schedule_training_job))
        .route("/{:job_id}/status", patch(update_training_job_status))
        .route("/{:job_id}/logs", post(post_logs))
}