pub mod handlers;
pub mod models;

use axum::{
    routing::{get, post},
    Router,
};

use crate::inbound::http::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::create_queue).get(handlers::list_queues))
        .route(
            "/{id}",
            get(handlers::get_queue)
                .put(handlers::update_queue)
                .delete(handlers::delete_queue),
        )
}