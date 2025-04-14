use axum::{
    body::Body,
    extract::Request,
    routing::{get, post},
    Extension, Router,
};
use common::database::Database;
use controlplane_api::routes;
use dotenv::dotenv;
use tower_http::trace::TraceLayer;
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

    // build our application with a route
    let app = Router::new()
        // user routes
        .route("/users", post(routes::create_user))
        .route("/users/{user_id}", get(routes::get_user))
        // auth routes
        .route("/auth/login", post(routes::authorize))
        // organizatino routes
        .route("/organization", get(routes::list_organizations))
        .route(
            "/organization/{organization_id}",
            get(routes::get_organization),
        )
        .route("/organization", post(routes::create_organization))
        // step definitions routes
        .route("/step_definitions", get(routes::list_step_definitions))
        // pipeline routes
        .route("/pipeline", post(routes::create_pipeline))
        .route("/pipeline/{pipeline_id}", get(routes::get_pipeline))
        .route(
            "/pipeline/{pipeline_id}/step/",
            post(routes::create_pipeline_step),
        )
        .route(
            "/pipeline/{pipeline_id}/step/{step_id}",
            get(routes::get_pipeline_step),
        )
        .route(
            "/pipeline/{pipeline_id}/connection/{from_step_id}/{to_step_id}",
            post(routes::connect_pipeline_step).delete(routes::disconnect_pipeline_step),
        )
        .layer(Extension(db))
        .layer(
            TraceLayer::new_for_http().make_span_with(|_request: &Request<Body>| {
                let request_id = Uuid::new_v4().to_string();
                tracing::info_span!("http-request", %request_id)
            }),
        );

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
