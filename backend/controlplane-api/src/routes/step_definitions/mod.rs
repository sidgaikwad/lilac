use axum::{routing::get, Router};
mod step_definitions;
use step_definitions::list_step_definitions;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/step_definitions", get(list_step_definitions))
}