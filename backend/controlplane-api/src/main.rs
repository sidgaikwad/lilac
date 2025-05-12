use axum::{body::Body, extract::{DefaultBodyLimit, Request}, http::HeaderValue, Router};
use common::{database::Database, s3::S3Wrapper};
use controlplane_api::{routes, AppState};
use dotenv::dotenv;
use hyper::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Method,
};
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

    let bucket_name = std::env::var("CUSTOMER_ASSETS_BUCKET").expect("CUSTOMER_ASSETS_BUCKET to be set");
    let s3 = S3Wrapper::new_from_default(bucket_name).await;
    let app = Router::new()
        .merge(routes::router())
        // Serve static files from the job_data directory
        // This path is relative to the WORKDIR in the Docker container (/usr/local/app)
        // and relies on the volume mount defined in docker-compose.dev.yml
        .nest_service("/static/job_outputs", ServeDir::new("/usr/local/app/data-pipeline/job_data"))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
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
        });

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
