use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use serde::{Deserialize, Serialize};

use headers::{authorization::Bearer, Authorization, HeaderMapExt};

use crate::domain::user::models::UserId;
use crate::inbound::http::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: UserId,
    pub exp: usize,
    pub iat: usize,
    pub jti: String,
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: UserId,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
}

impl Token {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Claims {
    pub sub: UserId,
    pub exp: usize,
    pub iat: usize,
}

impl FromRequestParts<AppState> for Claims {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // 2. Handle the Option returned by `typed_get` correctly.
        let bearer_token = parts
            .headers
            .typed_get::<Authorization<Bearer>>()
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header"))?;

        // 3. Validate the token using the application state
        let token_claims = state
            .auth_service
            .validate_token(bearer_token.token())
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;

        // 4. Create the Claims struct
        let claims = Claims {
            sub: token_claims.sub.into(),
            exp: token_claims.exp,
            iat: token_claims.iat,
        };

        Ok(claims)
    }
}
