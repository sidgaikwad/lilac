use common::database::Database;
use data_pipeline::get_steps_to_register;
use dotenv::dotenv;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

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

    let step_definitions = get_steps_to_register();
    for step_definition in step_definitions {
        db.register_step_definition(step_definition)
            .await
            .expect("step to be registered");
    }
}
