use std::sync::Arc;

use async_trait::async_trait;

use super::{
    models::{AuthUser, Token, TokenClaims},
    ports::{AuthService, TokenManager},
};
use crate::domain::user::{
    models::User,
    ports::{UserRepository, UserRepositoryError},
};

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("User not found")]
    UserNotFound,
    #[error("Password hashing failed")]
    PasswordHashingError,
    #[error("Internal error")]
    InternalError(#[from] anyhow::Error),
}

impl From<UserRepositoryError> for LoginError {
    fn from(err: UserRepositoryError) -> Self {
        match err {
            UserRepositoryError::NotFound(_) => LoginError::UserNotFound,
            _ => LoginError::InternalError(anyhow::anyhow!(err)),
        }
    }
}

pub struct AuthServiceImpl {
    user_repo: Arc<dyn UserRepository>,
    token_manager: Arc<dyn TokenManager>,
}

impl AuthServiceImpl {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        token_manager: Arc<dyn TokenManager>,
    ) -> Self {
        Self {
            user_repo,
            token_manager,
        }
    }

    async fn verify_password(&self, password: &str, user: &User) -> Result<(), LoginError> {
        let password_hash = user
            .password_hash
            .as_ref()
            .ok_or(LoginError::InvalidCredentials)?;
        if password_auth::verify_password(password, password_hash).is_err() {
            return Err(LoginError::PasswordHashingError);
        }

        Ok(())
    }

    // fn validate_token(&self, token: &str) -> Result<TokenClaims, anyhow::Error> {
    //     self.token_manager.validate_token(token)
    // }
}
#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn login_with_email(
        &self,
        email: &str,
        password: &str,
    ) -> Result<Token, LoginError> {
        let user = self.user_repo.get_user_by_email(email).await?;
        self.verify_password(password, &user).await?;

        let auth_user = AuthUser {
            id: user.id,
            username: user.name.clone(),
            email: user.email.clone(),
        };

        let token_str = self
            .token_manager
            .create_token(&auth_user)
            .map_err(|e| LoginError::InternalError(e))?;
        Ok(Token::new(token_str))
    }

    fn validate_token(&self, token: &str) -> Result<TokenClaims, anyhow::Error> {
        self.token_manager.validate_token(token)
    }
}