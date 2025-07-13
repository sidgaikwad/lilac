use sqlx::{postgres::PgPoolOptions, PgPool};

mod datasets;
mod integrations;
mod projects;
mod services;
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

    pub fn from_pool(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn migrate(&self) -> Result<(), sqlx::migrate::MigrateError> {
        sqlx::migrate!("./migrations").run(&self.pool).await
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("{entity_type} {entity_id} not found")]
    NotFound {
        entity_type: String,
        entity_id: String,
    },
    #[error("{entity_type} {entity_id} already exists")]
    Conflict {
        entity_type: String,
        entity_id: String,
    },
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
}

fn translate_sqlx_error(
    entity_type: String,
    entity_id: String,
    error: sqlx::Error,
) -> DatabaseError {
    match error {
        sqlx::Error::Database(err) if err.kind() == sqlx::error::ErrorKind::UniqueViolation => {
            DatabaseError::Conflict {
                entity_type,
                entity_id,
            }
        }
        sqlx::Error::RowNotFound => DatabaseError::NotFound {
            entity_type,
            entity_id,
        },
        e => e.into(),
    }
}
