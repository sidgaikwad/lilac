use async_trait::async_trait;
use std::sync::Arc;
use validator::Validate;

use super::{
    models::{CreateUserRequest, User, UserId},
    ports::{UserRepository, UserRepositoryError},
};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn create_user(&self, req: &CreateUserRequest) -> Result<User, UserRepositoryError>;
    async fn get_user_by_id(&self, id: &UserId) -> Result<User, UserRepositoryError>;
    async fn delete_user(
        &self,
        current_user_id: &UserId,
        target_user_id: &UserId,
    ) -> Result<(), UserRepositoryError>;
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
    async fn create_user(&self, req: &CreateUserRequest) -> Result<User, UserRepositoryError> {
        req.validate()
            .map_err(|e| UserRepositoryError::InvalidInput(e.to_string()))?;

        if let Ok(_user) = self.repo.get_user_by_email(&req.email).await {
            return Err(UserRepositoryError::Duplicate(
                "email".to_string(),
                req.email.clone(),
            ));
        }

        let mut req = req.clone();
        if req.name.is_none() {
            req.name = Some(req.email.split('@').next().unwrap_or("").to_string());
        }

        self.repo.create_user(&req).await
    }

    async fn get_user_by_id(&self, id: &UserId) -> Result<User, UserRepositoryError> {
        self.repo.get_user_by_id(id).await
    }

    async fn delete_user(
        &self,
        current_user_id: &UserId,
        target_user_id: &UserId,
    ) -> Result<(), UserRepositoryError> {
        if current_user_id != target_user_id {
            return Err(UserRepositoryError::Unauthorized);
        }
        self.repo.delete_user(target_user_id).await
    }
}
