use axum::extract::FromRef;
use common::{database::Database, s3::S3Wrapper};

pub mod auth;
pub mod routes;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Database,
    pub s3: S3Wrapper,
}

impl FromRef<AppState> for Database {
    fn from_ref(app_state: &AppState) -> Database {
        app_state.db.clone()
    }
}

impl FromRef<AppState> for S3Wrapper {
    fn from_ref(app_state: &AppState) -> S3Wrapper {
        app_state.s3.clone()
    }
}