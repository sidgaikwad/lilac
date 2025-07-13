use std::net::SocketAddr;

use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Request},
    http::HeaderValue,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;

use controlplane_api::{
    auth::keys::{Keys, KEYS},
    aws::S3Wrapper,
    database::Database,
    routes, AppState, LilacConfig,
};
use dotenv::dotenv;

use hyper::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Method,
};
use openidconnect::reqwest;
use rustls::crypto::ring::default_provider;
use time::Duration;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tower_sessions::{session_store::ExpiredDeletion, Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::{sqlx::PgPool, PostgresStore};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    // load .env and config files
    dotenv().ok();
    let config = LilacConfig::new().expect("failed to parse config");
    KEYS.get_or_init(|| Keys::new(config.secret_key.as_bytes()));

    default_provider().install_default().unwrap();

    tracing::info!("database url: {}", config.database_url);
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("database pool to connect");

    let db = Database::from_pool(pool.clone());
    db.migrate().await.expect("migrations to complete");

    let session_store = PostgresStore::new(pool.clone());
    session_store
        .migrate()
        .await
        .expect("session store to migrate");

    tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::minutes(10)));

    let s3 = S3Wrapper::new_from_default().await;

    let mut headers = hyper::HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("lilac"));
    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .default_headers(headers)
        .build()
        .expect("Client should build");

    let app = Router::new()
        .merge(routes::router())
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_origin(config.frontend_url.parse::<HeaderValue>().unwrap())
                .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
                .allow_credentials(true),
        )
        .layer(session_layer)
        .layer(
            TraceLayer::new_for_http().make_span_with(|_request: &Request<Body>| {
                let request_id = Uuid::new_v4().to_string();
                tracing::info_span!("http-request", %request_id)
            }),
        )
        .layer(DefaultBodyLimit::max(20_000_000))
        .with_state(AppState {
            db,
            s3,
            http_client,
        });

    let addr = SocketAddr::from(([0, 0, 0, 0], config.http_port));
    match config.tls {
        Some(tls_cfg) => {
            let tls_config = RustlsConfig::from_pem_file(tls_cfg.cert_file, tls_cfg.key_file)
                .await
                .unwrap();
            tracing::info!("listening on https://{}", addr);
            axum_server::bind_rustls(addr, tls_config)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
        None => {
            tracing::info!("listening on http://{}", addr);
            axum_server::bind(addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
    }
}
