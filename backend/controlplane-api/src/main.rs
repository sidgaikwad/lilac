use std::collections::HashMap;

use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Request},
    http::HeaderValue,
    Router,
};
use common::{
    aws::{S3Wrapper, STSWrapper},
    database::Database,
};
use controlplane_api::{routes, AppState, Oauth2Config, OidcConfig};
use dotenv::dotenv;
use hyper::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Method,
};
use openidconnect::{
    core::CoreProviderMetadata, ClientId as OidcClientId, ClientSecret as OidcClientSecret,
    IssuerUrl, RedirectUrl as OidcRedirectUrl,
};
use openidconnect::reqwest;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl,
};
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;
use time::Duration;
use tokio::signal;
use tower_sessions::{session_store::ExpiredDeletion, Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::{sqlx::PgPool, PostgresStore};

#[tokio::main]
async fn main() {
    tracing::info!("starting app");
    // load .env
    dotenv().ok();

    tracing::info!("dotenv loaded");

    // initialize tracing
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into()))
        .init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL to be set");

    tracing::info!("database url: {}", db_url);
    let db = Database::new(&db_url).await.expect("database to connect");
    db.migrate().await.expect("migrations to complete");

    let pool = PgPool::connect(&db_url).await.expect("database pool to connect");
    let session_store = PostgresStore::new(pool.clone());
    session_store.migrate().await.expect("session store to migrate");

    let deletion_task = tokio::task::spawn(
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

    let mut headers = hyper::HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("controlplane-api"),
    );
    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .default_headers(headers)
        .build()
        .expect("Client should build");

    let mut oidc_configs = HashMap::new();
    let supported_providers = ["google"];

    let frontend_redirect_url = std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:5173".to_string());

    for provider_name in supported_providers.iter() {
        let client_id_var = format!("{}_CLIENT_ID", provider_name.to_uppercase());
        let client_secret_var = format!("{}_CLIENT_SECRET", provider_name.to_uppercase());
        let issuer_url_var = format!("{}_ISSUER_URL", provider_name.to_uppercase());

        let oidc_redirect_url = OidcRedirectUrl::new(format!(
            "{}/auth/callback/oidc/{}",
            frontend_redirect_url, provider_name
        ))
        .expect("Invalid OIDC redirect URL");

        if let (Ok(client_id), Ok(client_secret), Ok(issuer_url)) = (
            std::env::var(&client_id_var),
            std::env::var(&client_secret_var),
            std::env::var(&issuer_url_var),
        ) {
            let provider_metadata =
                CoreProviderMetadata::discover_async(IssuerUrl::new(issuer_url).unwrap(), &http_client)
                    .await
                    .unwrap();

            oidc_configs.insert(
                provider_name.to_string(),
                OidcConfig {
                    provider_metadata,
                    client_id: OidcClientId::new(client_id),
                    client_secret: Some(OidcClientSecret::new(client_secret)),
                    redirect_uri: oidc_redirect_url.clone(),
                    frontend_redirect_url: frontend_redirect_url.clone(),
                },
            );
            tracing::info!("Successfully configured OIDC provider: {}", provider_name);
        }
    }

    let mut oauth2_configs = HashMap::new();
    let supported_oauth2_providers = ["github"];

    for provider_name in supported_oauth2_providers.iter() {
        let client_id_var = format!("{}_CLIENT_ID", provider_name.to_uppercase());
        let client_secret_var = format!("{}_CLIENT_SECRET", provider_name.to_uppercase());
        let auth_url_var = format!("{}_AUTH_URL", provider_name.to_uppercase());
        let token_url_var = format!("{}_TOKEN_URL", provider_name.to_uppercase());
        let user_info_url_var = format!("{}_USER_INFO_URL", provider_name.to_uppercase());

        let oauth2_redirect_url = RedirectUrl::new(format!(
            "{}/auth/callback/oauth2/{}",
            frontend_redirect_url, provider_name
        ))
        .expect("Invalid OAuth2 redirect URL");

        if let (Ok(client_id), Ok(client_secret), Ok(auth_url), Ok(token_url), Ok(user_info_url)) = (
            std::env::var(&client_id_var),
            std::env::var(&client_secret_var),
            std::env::var(&auth_url_var),
            std::env::var(&token_url_var),
            std::env::var(&user_info_url_var),
        ) {
            oauth2_configs.insert(
                provider_name.to_string(),
                Oauth2Config {
                    client_id: ClientId::new(client_id),
                    client_secret: ClientSecret::new(client_secret),
                    auth_url: AuthUrl::new(auth_url).expect("Invalid auth URL"),
                    token_url: TokenUrl::new(token_url).expect("Invalid token URL"),
                    redirect_url: oauth2_redirect_url.clone(),
                    user_info_url,
                    frontend_redirect_url: frontend_redirect_url.clone(),
                },
            );
            tracing::info!("Successfully configured OAuth2 provider: {}", provider_name);
        }
    }

    let app = Router::new()
        .merge(routes::router())
        .nest_service(
            "/static/job_outputs",
            ServeDir::new("/usr/local/app/data-pipeline/job_data"),
        )
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_origin(
                    std::env::var("FRONTEND_URL")
                        .unwrap_or_else(|_| "http://localhost:5173".to_string())
                        .parse::<HeaderValue>()
                        .unwrap(),
                )
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
        });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
        .await
        .unwrap();
}

async fn shutdown_signal(deletion_task_abort_handle: tokio::task::AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}
