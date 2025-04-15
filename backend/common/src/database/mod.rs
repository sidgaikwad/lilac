use sqlx::{postgres::PgPoolOptions, PgPool};

mod organization;
mod pipeline;
mod step_definitions;
mod steps;
mod user;
mod jobs;

#[derive(Clone, Debug)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new().max_connections(5).connect(url).await?;
        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<(), sqlx::migrate::MigrateError> {
        sqlx::migrate!("./migrations").run(&self.pool).await
    }
}
