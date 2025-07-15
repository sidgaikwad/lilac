use async_trait::async_trait;

use super::{
    models::{AuthUser, Token, TokenClaims},
    service::LoginError,
};

pub trait TokenManager: Send + Sync {
    fn create_token(&self, user: &AuthUser) -> Result<String, anyhow::Error>;
    fn validate_token(&self, token: &str) -> Result<TokenClaims, anyhow::Error>;
}

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn login_with_email(&self, email: &str, password: &str) -> Result<Token, LoginError>;
    fn validate_token(&self, token: &str) -> Result<TokenClaims, anyhow::Error>;
}
