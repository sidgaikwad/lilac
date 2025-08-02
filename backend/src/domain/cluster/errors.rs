use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClusterApiKeyRepositoryError {
    #[error("api key not found")]
    NotFound,
    #[error("database error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("unknown error")]
    Unknown(#[from] anyhow::Error),
}
