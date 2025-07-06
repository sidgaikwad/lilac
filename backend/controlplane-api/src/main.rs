use std::{collections::HashMap, net::SocketAddr};

use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Request},
    http::HeaderValue,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use common::{
    aws::{S3Wrapper, STSWrapper},
    database::Database,
    k8s::K8sWrapper,
};
use controlplane_api::{auth::keys::{Keys, KEYS}, routes, AppState, LilacConfig, Oauth2Config, OidcConfig, SsoConfig};
use dotenv::dotenv;

use hyper::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Method,
};
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use openidconnect::reqwest;
use openidconnect::{
    core::CoreProviderMetadata, ClientId as OidcClientId, ClientSecret as OidcClientSecret,
    IssuerUrl, RedirectUrl as OidcRedirectUrl,
};
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
        .with_env_filter(EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy())
        .init();

    // load .env and config files
    dotenv().ok();
    let config = LilacConfig::new().expect("failed to parse config");
    KEYS.get_or_init(|| {
        Keys::new(config.secret_key.as_bytes())
    });

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

    let bucket_name =
        std::env::var("CUSTOMER_ASSETS_BUCKET").expect("CUSTOMER_ASSETS_BUCKET to be set");
    let s3 = S3Wrapper::new_from_default(bucket_name).await;
    let sts = STSWrapper::new_from_default().await;
    let k8s = K8sWrapper::new(String::new()).await;

    let mut headers = hyper::HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("lilac"));
    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .default_headers(headers)
        .build()
        .expect("Client should build");

    let mut oidc_configs = HashMap::new();
    let mut oauth2_configs = HashMap::new();

    for (provider_name, sso_cfg) in config.sso.clone().into_iter() {
        match sso_cfg {
            SsoConfig::Oidc {
                client_id,
                client_secret,
                issuer_url,
            } => {
                let oidc_redirect_url = OidcRedirectUrl::new(format!(
                    "{}/auth/callback/oidc/{}",
                    config.frontend_url, provider_name
                ))
                .expect("Invalid OIDC redirect URL");
                let provider_metadata = CoreProviderMetadata::discover_async(
                    IssuerUrl::new(issuer_url).unwrap(),
                    &http_client,
                )
                .await
                .unwrap();

                oidc_configs.insert(
                    provider_name.to_string(),
                    OidcConfig {
                        provider_metadata,
                        client_id: OidcClientId::new(client_id),
                        client_secret: Some(OidcClientSecret::new(client_secret)),
                        redirect_uri: oidc_redirect_url.clone(),
                        frontend_redirect_url: config.frontend_url.clone(),
                    },
                );
                tracing::info!("Successfully configured OIDC provider: {}", provider_name);
            }
            SsoConfig::Oauth2 {
                client_id,
                client_secret,
                auth_url,
                token_url,
                user_info_url,
            } => {
                let oauth2_redirect_url = RedirectUrl::new(format!(
                    "{}/auth/callback/oauth2/{}",
                    config.frontend_url, provider_name
                ))
                .expect("Invalid OAuth2 redirect URL");
                oauth2_configs.insert(
                    provider_name.to_string(),
                    Oauth2Config {
                        client_id: ClientId::new(client_id),
                        client_secret: ClientSecret::new(client_secret),
                        auth_url: AuthUrl::new(auth_url).expect("Invalid auth URL"),
                        token_url: TokenUrl::new(token_url).expect("Invalid token URL"),
                        redirect_url: oauth2_redirect_url.clone(),
                        user_info_url,
                        frontend_redirect_url: config.frontend_url.clone(),
                    },
                );
                tracing::info!("Successfully configured Oauth2 provider: {}", provider_name);
            }
        }
    }

    let service_config = ServiceConfig {
        gateway_name: std::env::var("GATEWAY_NAME").unwrap_or_else(|_| "lilac-gateway".to_string()),
        gateway_namespace: std::env::var("GATEWAY_NAMESPACE")
            .unwrap_or_else(|_| "lilac-system".to_string()),
        gateway_url: std::env::var("GATEWAY_URL").unwrap_or_else(|_| "http://localhost:30080".to_string()),
    };

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
            sts,
            oidc_configs,
            oauth2_configs,
            http_client,
            k8s,
            service_config,
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
