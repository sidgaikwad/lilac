use async_trait::async_trait;
use thiserror::Error;

use super::models::{CreateUserRequest, User, UserId};

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("{0} with value {1} already exists")]
    Duplicate(String, String),
    #[error("user with id {0} not found")]
    NotFound(String),
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create_user(&self, req: &CreateUserRequest) -> Result<User, UserRepositoryError>;
    async fn get_user_by_id(&self, id: &UserId) -> Result<User, UserRepositoryError>;
    async fn get_user_by_email(&self, email: &str) -> Result<User, UserRepositoryError>;
    async fn delete_user(&self, id: &UserId) -> Result<(), UserRepositoryError>;
}