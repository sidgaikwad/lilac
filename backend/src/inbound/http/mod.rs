pub mod errors;
pub mod routes;

use axum::{extract::FromRef, Router};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_sessions::{service::PrivateCookie, SessionManagerLayer};

use crate::{
    config::LilacConfig,
    domain::{
        auth::service::AuthService, cluster::service::ClusterService,
        credentials::service::CredentialService, dataset::service::DatasetService,
        project::service::ProjectService, user::service::UserService,
    },
    inbound::http::routes::{clusters, credentials},
    outbound::persistence::postgres::session_repository::PostgresSessionStore,
};

use self::routes::{auth, datasets, projects, users};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<LilacConfig>,
    pub cluster_service: Arc<dyn ClusterService>,
    pub credential_service: Arc<dyn CredentialService>,
    pub user_service: Arc<dyn UserService>,
    pub project_service: Arc<dyn ProjectService>,
    pub dataset_service: Arc<dyn DatasetService>,
    pub auth_service: Arc<dyn AuthService>,
}

impl FromRef<AppState> for Arc<dyn CredentialService> {
    fn from_ref(state: &AppState) -> Self {
        state.credential_service.clone()
    }
}

impl FromRef<AppState> for Arc<dyn ClusterService> {
    fn from_ref(state: &AppState) -> Self {
        state.cluster_service.clone()
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

impl FromRef<AppState> for Arc<dyn AuthService> {
    fn from_ref(state: &AppState) -> Self {
        state.auth_service.clone()
    }
}

pub struct HttpServer {
    app: Router,
    listener: TcpListener,
    address: String,
    port: u16,
}

impl HttpServer {
    pub async fn new(
        app_state: AppState,
        session_layer: SessionManagerLayer<PostgresSessionStore, PrivateCookie>,
        port: u16,
    ) -> anyhow::Result<Self> {
        let app: Router = Router::new()
            .merge(users::router())
            .merge(projects::router())
            .merge(datasets::router())
            .merge(auth::router())
            .merge(clusters::router())
            .merge(credentials::router())
            .layer(session_layer)
            .with_state(app_state);

        let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
        Ok(Self {
            app,
            listener,
            address: "0.0.0.0".into(),
            port,
        })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        tracing::info!("starting server on {}:{}", self.address, self.port);
        axum::serve(self.listener, self.app).await?;
        Ok(())
    }
}
