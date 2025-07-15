pub mod requests;
pub mod responses;
pub mod routes;

use std::sync::Arc;
use axum::{
    extract::FromRef,
    Router,
};
use tokio::net::TcpListener;

use crate::{config::LilacConfig, domain::{
    auth::ports::AuthService,
    dataset::ports::DatasetService,
    integration::{ports::StsPort, service::IntegrationService},
    project::{ports::ProjectRepository, service::ProjectService},
    service::ports::ServiceService,
    user::service::UserService,
}};
// use tower_sessions_sqlx_store::PostgresStore;

use self::routes::{auth, dataset, integration, project, service, user};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<LilacConfig>,
    pub user_service: Arc<dyn UserService>,
    pub project_service: Arc<dyn ProjectService>,
    pub dataset_service: Arc<dyn DatasetService>,
    pub integration_service: Arc<dyn IntegrationService>,
    pub service_service: Arc<dyn ServiceService>,
    pub auth_service: Arc<dyn AuthService>,
    pub project_repo: Arc<dyn ProjectRepository>,
    pub sts_port: Arc<dyn StsPort>,
}

impl FromRef<AppState> for Arc<dyn StsPort> {
    fn from_ref(state: &AppState) -> Self {
        state.sts_port.clone()
    }
}

impl FromRef<AppState> for Arc<dyn UserService> {
    fn from_ref(state: &AppState) -> Self {
        state.user_service.clone()
    }
}

impl FromRef<AppState> for Arc<dyn ProjectService> {
    fn from_ref(state: &AppState) -> Self {
        state.project_service.clone()
    }
}

impl FromRef<AppState> for Arc<dyn DatasetService> {
    fn from_ref(state: &AppState) -> Self {
        state.dataset_service.clone()
    }
}

impl FromRef<AppState> for Arc<dyn IntegrationService> {
    fn from_ref(state: &AppState) -> Self {
        state.integration_service.clone()
    }
}

impl FromRef<AppState> for Arc<dyn ServiceService> {
    fn from_ref(state: &AppState) -> Self {
        state.service_service.clone()
    }
}

impl FromRef<AppState> for Arc<dyn AuthService> {
    fn from_ref(state: &AppState) -> Self {
        state.auth_service.clone()
    }
}

pub struct HttpServer {
    app: Router,
    listener: TcpListener,
}

impl HttpServer {
    pub async fn new(
        app_state: AppState,
        port: u16,
    ) -> anyhow::Result<Self> {
        let app: Router = Router::new()
            .merge(user::router())
            .merge(project::router())
            .merge(dataset::router())
            .merge(integration::routes())
            .merge(service::routes())
            .merge(auth::router())
            .with_state(app_state);

        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        Ok(Self { app, listener })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        axum::serve(self.listener, self.app).await?;
        Ok(())
    }
}