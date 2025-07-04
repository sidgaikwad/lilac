use axum::{
    routing::{get, post},
    Router,
};
mod user;
use crate::AppState;
use user::{create_user, delete_user_handler, get_current_user, get_user};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/account/details", get(get_current_user))
        .route(
            "/users/{user_id}",
            get(get_user).delete(delete_user_handler),
        )
        .route("/auth/signup", post(create_user))
}
