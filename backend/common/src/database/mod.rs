use sqlx::{postgres::PgPoolOptions, PgPool};

mod jobs;
mod organizations;
mod pipelines;
mod projects;
mod step_definitions;
mod steps;
mod users;

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
