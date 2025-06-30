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
use controlplane_api::{routes, AppState, OidcConfig};
use dotenv::dotenv;
use hyper::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use openidconnect::{
    core::CoreProviderMetadata, ClientId, ClientSecret, IssuerUrl, RedirectUrl,
};
use openidconnect::reqwest;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer}; // Added ServeDir
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

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

    let bucket_name =
        std::env::var("CUSTOMER_ASSETS_BUCKET").expect("CUSTOMER_ASSETS_BUCKET to be set");
    let s3 = S3Wrapper::new_from_default(bucket_name).await;
    let sts = STSWrapper::new_from_default().await;

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    let mut oidc_configs = HashMap::new();
    let supported_providers = ["google", "gitlab", "okta"];

    let redirect_url = RedirectUrl::new(
        std::env::var("REDIRECT_URL").unwrap_or("http://localhost:3000/auth/oidc/callback".to_string()),
    )
    .expect("Invalid redirect URL");

    let frontend_redirect_url = std::env::var("FRONTEND_REDIRECT_URL")
        .unwrap_or_else(|_| "http://localhost:5173/auth/callback".to_string());

    for provider_name in supported_providers.iter() {
        let client_id_var = format!("{}_CLIENT_ID", provider_name.to_uppercase());
        let client_secret_var = format!("{}_CLIENT_SECRET", provider_name.to_uppercase());
        let issuer_url_var = format!("{}_ISSUER_URL", provider_name.to_uppercase());

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
                    client_id: ClientId::new(client_id),
                    client_secret: Some(ClientSecret::new(client_secret)),
                    redirect_uri: redirect_url.clone(),
                    frontend_redirect_url: frontend_redirect_url.clone(),
                },
            );
            tracing::info!("Successfully configured OIDC provider: {}", provider_name);
        }
    }

    let app = Router::new()
        .merge(routes::router())
        // Serve static files from the job_data directory
        // This path is relative to the WORKDIR in the Docker container (/usr/local/app)
        // and relies on the volume mount defined in docker-compose.dev.yml
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
            http_client,
        });

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
