use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::user::UserId;

use super::{error::AuthError, keys::KEYS};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub sub: UserId,
    pub jti: String,
}

impl Claims {
    pub fn new(sub: UserId, exp: usize, iat: usize, jti: String) -> Self {
        Self { sub, exp, iat, jti }
    }

    pub fn create(user_id: UserId) -> Self {
        let now = Utc::now();
        let exp = (now + Duration::hours(6)).timestamp() as usize;
        Self {
            sub: user_id,
            exp,
            iat: now.timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
        }
    }
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingCredentials)?;
        // Decode the user data
        let token_data = decode::<Claims>(
            bearer.token(),
            &KEYS.get().unwrap().decoding,
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
