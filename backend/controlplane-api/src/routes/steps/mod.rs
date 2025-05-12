use axum::{
    routing::{get, post},
    Router,
};

mod steps;
use steps::*;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/steps", post(create_step)).route(
        "/steps/{step_id}",
        get(get_step).delete(delete_step).post(update_step),
    )
}
