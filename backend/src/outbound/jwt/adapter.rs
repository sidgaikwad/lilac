use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::domain::auth::{
    models::{AuthUser, TokenClaims},
    ports::TokenManager,
};

pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtManager {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }
}

#[async_trait]
impl TokenManager for JwtManager {
    fn create_token(&self, user: &AuthUser) -> Result<String, anyhow::Error> {
        let now = Utc::now();
        let claims = TokenClaims {
            sub: user.id,
            exp: (now + Duration::hours(6)).timestamp() as usize,
            iat: now.timestamp() as usize,
            jti: uuid::Uuid::new_v4().to_string(),
        };

        encode(&Header::default(), &claims, &self.encoding_key).map_err(anyhow::Error::from)
    }

    fn validate_token(&self, token: &str) -> Result<TokenClaims, anyhow::Error> {
        decode::<TokenClaims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(anyhow::Error::from)
    }
}
