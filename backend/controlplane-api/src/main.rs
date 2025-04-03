use axum::{
    routing::{get, post}, Extension, Router
};
use controlplane_api::routes;
use sqlx::postgres::PgPoolOptions;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    tracing::info!("starting app");
    // load .env
    dotenv().ok();

    tracing::info!("dotenv loaded");

    // initialize tracing
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into()))
        .init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL to be set");

    tracing::info!("database url: {}", db_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await
        .expect("database to connect");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await.unwrap();

    // build our application with a route
    let app = Router::new()
        // `POST /users` goes to `create_user`
        .route("/users", post(routes::create_user))
        .route("/users/{user_id}", get(routes::get_user))
        .route("/auth/login", post(routes::authorize))
        .layer(Extension(pool));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
