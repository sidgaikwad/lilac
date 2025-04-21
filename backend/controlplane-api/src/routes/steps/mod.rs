use axum::{
    routing::{get, post},
    Router,
};

mod steps;
use steps::*;

pub fn router() -> Router {
    Router::new().route("/steps", post(create_step)).route(
        "/steps/{step_id}",
        get(get_step).delete(delete_step).post(update_step),
    )
}
