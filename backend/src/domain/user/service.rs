use async_trait::async_trait;
use std::sync::Arc;

use super::{
    models::{CreateUserRequest, User, UserId},
    ports::{UserRepository, UserRepositoryError},
};

#[derive(Debug, thiserror::Error)]
pub enum UserServiceError {
    #[error("invalid permissions")]
    InvalidPermissions,
    #[error("user with {field} {value} already exists")]
    UserExists { field: String, value: String },
    #[error("user {0} not found")]
    UserNotFound(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<UserRepositoryError> for UserServiceError {
    fn from(error: UserRepositoryError) -> Self {
        match error {
            UserRepositoryError::Duplicate { field, value } => Self::UserExists { field, value },
            UserRepositoryError::NotFound(id) => Self::UserNotFound(id),
            UserRepositoryError::Unknown(error) => Self::Unknown(error),
        }
    }
}

#[async_trait]
pub trait UserService: Send + Sync {
    async fn create_user(&self, req: &CreateUserRequest) -> Result<User, UserServiceError>;
    async fn get_user_by_id(&self, id: &UserId) -> Result<User, UserServiceError>;
    async fn delete_user(
        &self,
        current_user_id: &UserId,
        target_user_id: &UserId,
    ) -> Result<(), UserServiceError>;
}

#[derive(Clone)]
pub struct UserServiceImpl<R: UserRepository> {
    repo: Arc<R>,
}

impl<R: UserRepository> UserServiceImpl<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: UserRepository> UserService for UserServiceImpl<R> {
    async fn create_user(&self, req: &CreateUserRequest) -> Result<User, UserServiceError> {
        let mut req = req.clone();
        if req.name.is_none() {
            req.name = Some(req.email.split('@').next().unwrap_or("").to_string());
        }

        Ok(self.repo.create_user(&req).await?)
    }

    async fn get_user_by_id(&self, id: &UserId) -> Result<User, UserServiceError> {
        Ok(self.repo.get_user_by_id(id).await?)
    }

    async fn delete_user(
        &self,
        current_user_id: &UserId,
        target_user_id: &UserId,
    ) -> Result<(), UserServiceError> {
        if current_user_id != target_user_id {
            return Err(UserServiceError::InvalidPermissions);
        }
        Ok(self.repo.delete_user(target_user_id).await?)
    }
}
