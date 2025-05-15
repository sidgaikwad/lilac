use axum::{routing::{get, post}, Router};
mod user;
use user::{create_user, get_current_user, get_user};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/account/details", get(get_current_user))
        .route("/users", post(create_user))
        .route("/users/{user_id}", get(get_user))
}