pub mod handlers;
pub mod models;

use axum::Router;

use crate::inbound::http::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
    // .route(
    //     "/",
    //     post(handlers::create_workspace_handler).get(handlers::list_workspaces_handler),
    // )
    // .route(
    //     "/{:workspace_id}/connection",
    //     get(handlers::get_workspace_connection_handler),
    // )
}
