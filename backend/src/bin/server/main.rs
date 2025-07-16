use secrecy::ExposeSecret;
use std::sync::Arc;
use tower_sessions::{cookie::Key, Expiry, SessionManagerLayer};

use server::{
    config::{LilacConfig, LogFormat},
    domain::{
        auth::service::AuthServiceImpl, dataset::service::DatasetServiceImpl,
        project::service::ProjectServiceImpl, user::service::UserServiceImpl,
    },
    inbound::http::{AppState, HttpServer},
    outbound::{
        data_source::adapter::DataSourceTesterImpl,
        jwt::JwtManager,
        persistence::postgres::{
            dataset_repository::PostgresDatasetRepository,
            project_repository::PostgresProjectRepository,
            session_repository::PostgresSessionStore, user_repository::PostgresUserRepository,
        },
    },
};
use sqlx::postgres::PgPoolOptions;
// use tower_sessions::{cookie::Key, Expiry, SessionManagerLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 0. Install the default crypto provider
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install the default crypto provider");

    // 1. Load config
    let config = Arc::new(LilacConfig::new().expect("could not parse provided config"));

    // 2. Initialize tracing
    let trace_subscriber = tracing_subscriber::registry().with(
        EnvFilter::builder()
            .with_default_directive((&config.log_level).into())
            .from_env_lossy(),
    );
    match config.log_format {
        LogFormat::Pretty => trace_subscriber
            .with(tracing_subscriber::fmt::layer().pretty())
            .try_init()
            .expect("no tracing subscriber to already be installed"),
        LogFormat::Json => trace_subscriber
            .with(tracing_subscriber::fmt::layer().json())
            .try_init()
            .expect("no tracing subscriber to already be installed"),
    };

    // 3. Construct outbound adapters
    let db_pool = PgPoolOptions::new()
        .connect(config.database_url.expose_secret())
        .await?;
    let user_repo = Arc::new(PostgresUserRepository::new(db_pool.clone()));
    let project_repo = Arc::new(PostgresProjectRepository::new(db_pool.clone()));
    let dataset_repo = Arc::new(PostgresDatasetRepository::new(db_pool.clone()));
    let jwt_manager = Arc::new(JwtManager::new(config.secret_key.expose_secret()));

    // 3. Construct domain services
    let user_service = Arc::new(UserServiceImpl::new(user_repo.clone()));
    let project_service = Arc::new(ProjectServiceImpl::new(project_repo.clone()));
    let dataset_service = Arc::new(DatasetServiceImpl::new(
        dataset_repo.clone(),
        Arc::new(DataSourceTesterImpl),
    ));
    let session_store = PostgresSessionStore::new(db_pool.clone());
    session_store.migrate().await?;
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        .with_private(Key::from(config.secret_key.expose_secret().as_bytes()))
        .with_expiry(Expiry::OnInactivity(time::Duration::minutes(30)));

    let auth_service = Arc::new(AuthServiceImpl::new(user_repo.clone(), jwt_manager));

    // 4. Construct and run inbound adapter (HTTP server)
    let app_state = AppState {
        config: config.clone(),
        user_service,
        project_service,
        dataset_service,
        auth_service,
    };
    let http_server = HttpServer::new(app_state, session_layer, config.http_port).await?;
    http_server.run().await
}
