use super::models::{AuthUser, TokenClaims};

pub trait TokenManager: Send + Sync {
    fn create_token(&self, user: &AuthUser) -> Result<String, anyhow::Error>;
    fn validate_token(&self, token: &str) -> Result<TokenClaims, anyhow::Error>;
}
