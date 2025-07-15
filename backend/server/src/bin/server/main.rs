use std::sync::Arc;
use secrecy::ExposeSecret;

use server::{
    config::LilacConfig,
    domain::{
        auth::service::AuthServiceImpl,
        dataset::service::DatasetServiceImpl,
        integration::{service::IntegrationServiceImpl},
        project::service::ProjectServiceImpl,
        service::service::ServiceServiceImpl,
        user::service::UserServiceImpl,
    },
    inbound::http::{AppState, HttpServer},
    outbound::{
        jwt::JwtManager,
        persistence::postgres::{
            dataset_repository::PostgresDatasetRepository,
            integration_repository::PostgresIntegrationRepository,
            project_repository::PostgresProjectRepository,
            service_repository::PostgresServiceRepository,
            user_repository::PostgresUserRepository,
        },
        data_source::adapter::DataSourceImpl,
        k8s::adapter::K8sAdapter,
        sts::adapter::StsAdapter,
    },
};
use sqlx::postgres::PgPoolOptions;
// use tower_sessions::{cookie::Key, Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 0. Install the default crypto provider
    rustls::crypto::ring::default_provider().install_default()
        .expect("Failed to install the default crypto provider");
    
    // 1. Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_target(false)
        .init();

    // 2. Load config
    let config = LilacConfig::new().unwrap();

    // 2. Construct outbound adapters
    let db_pool = PgPoolOptions::new().connect(&config.database_url.expose_secret()).await?;
    let user_repo = Arc::new(PostgresUserRepository::new(db_pool.clone()));
    let project_repo = Arc::new(PostgresProjectRepository::new(db_pool.clone()));
    let dataset_repo = Arc::new(PostgresDatasetRepository::new(db_pool.clone()));
    let integration_repo = Arc::new(PostgresIntegrationRepository::new(db_pool.clone()));
    let service_repo = Arc::new(PostgresServiceRepository::new(db_pool.clone()));
    let jwt_manager = Arc::new(JwtManager::new(config.secret_key.expose_secret()));
    let sts_adapter = Arc::new(StsAdapter::new().await);

    // 3. Construct domain services
    let user_service = Arc::new(UserServiceImpl::new(user_repo.clone()));
    let project_service = Arc::new(ProjectServiceImpl::new(project_repo.clone()));
    let dataset_service = Arc::new(DatasetServiceImpl::new(
        dataset_repo.clone(),
        Arc::new(DataSourceImpl),
    ));
    let k8s_adapter = Arc::new(K8sAdapter::new().await);
    let integration_service = Arc::new(IntegrationServiceImpl::new(
        integration_repo.clone(),
        sts_adapter.clone(),
        k8s_adapter,
    ));
    let service_service = Arc::new(ServiceServiceImpl::new(service_repo.clone(), project_repo.clone()));
    let session_store = PostgresStore::new(db_pool.clone());
    session_store.migrate().await?;
    // let session_layer = SessionManagerLayer::new(session_store)
    //     .with_secure(true)
    //     .with_private(Key::from(config.secret_key.expose_secret().as_bytes()))
    //     .with_expiry(Expiry::OnInactivity(time::Duration::days(1)));

    let auth_service = Arc::new(AuthServiceImpl::new(
        user_repo.clone(),
        jwt_manager,
    ));

    // 4. Construct and run inbound adapter (HTTP server)
    let app_state = AppState {
        user_service,
        project_service,
        dataset_service,
        integration_service,
        service_service,
        auth_service,
        project_repo: project_repo.clone(),
        sts_port: sts_adapter.clone(),
    };
    let http_server = HttpServer::new(app_state, config.http_port).await?;
    http_server.run().await
}