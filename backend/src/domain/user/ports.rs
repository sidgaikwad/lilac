use async_trait::async_trait;
use thiserror::Error;

use super::models::{CreateUserRequest, User, UserId};

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("user with {field} {value} already exists")]
    Duplicate { field: String, value: String },
    #[error("user with id {0} not found")]
    NotFound(String),
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
