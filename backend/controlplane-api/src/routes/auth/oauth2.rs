use super::sso;
use crate::{auth::error::AuthError, AppState};
use axum::{
    extract::{Path, State},
    Json,
};
use oauth2::{
    basic::BasicClient, AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope,
    TokenResponse,
};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    authorization_url: String,
}

#[tracing::instrument(level = "debug", skip(app_state), ret, err)]
pub async fn login(
    State(app_state): State<AppState>,
    Path(provider): Path<String>,
    session: Session,
) -> Result<Json<LoginResponse>, AuthError> {
    let config = app_state
        .oauth2_configs
        .get(&provider)
        .ok_or(AuthError::ProviderNotFound)?;

    let client = BasicClient::new(config.client_id.clone())
        .set_client_secret(config.client_secret.clone())
        .set_auth_uri(config.auth_url.clone())
        .set_token_uri(config.token_url.clone())
        .set_redirect_uri(config.redirect_url.clone());

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user:email".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();
    tracing::debug!("Generated authorization URL: {}", auth_url);
    session
        .insert("pkce_verifier", pkce_verifier)
        .await
        .map_err(|_| AuthError::SessionError)?;
    session
        .insert("csrf_token", csrf_token.secret().clone())
        .await
        .map_err(|_| AuthError::SessionError)?;

    Ok(Json(LoginResponse {
        authorization_url: auth_url.to_string(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct ExchangePayload {
    code: String,
    state: String,
}

#[derive(Debug, Serialize)]
pub struct ExchangeResponse {
    access_token: String,
}

#[tracing::instrument(level = "debug", skip(app_state), ret, err)]
pub async fn exchange(
    State(app_state): State<AppState>,
    Path(provider): Path<String>,
    session: Session,
    Json(payload): Json<ExchangePayload>,
) -> Result<Json<ExchangeResponse>, AuthError> {
    tracing::debug!("get pkce verifier from session");
    let pkce_verifier: PkceCodeVerifier = session
        .get("pkce_verifier")
        .await
        .map_err(|_| AuthError::SessionError)?
        .ok_or(AuthError::SessionError)?;
    tracing::debug!("Retrieving csrf token from session");
    let csrf_token: String = session
        .get("csrf_token")
        .await
        .map_err(|_| AuthError::SessionError)?
        .ok_or(AuthError::SessionError)?;

    if csrf_token != payload.state {
        return Err(AuthError::CsrfMismatch);
    }

    let config = app_state
        .oauth2_configs
        .get(&provider)
        .ok_or(AuthError::ProviderNotFound)?;

    let client = BasicClient::new(config.client_id.clone())
        .set_client_secret(config.client_secret.clone())
        .set_auth_uri(config.auth_url.clone())
        .set_token_uri(config.token_url.clone())
        .set_redirect_uri(config.redirect_url.clone());

    tracing::debug!("Exchanging code for token");
    let token_response = client
        .exchange_code(AuthorizationCode::new(payload.code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(&app_state.http_client)
        .await
        .map_err(|_| AuthError::CodeExchangeFailed)?;

    tracing::debug!("Getting user info");
    let user_info: serde_json::Value = app_state
        .http_client
        .get(&config.user_info_url)
        .bearer_auth(token_response.access_token().secret())
        .send()
        .await
        .map_err(|_| AuthError::ClaimsVerificationFailed)?
        .json()
        .await
        .map_err(|_| AuthError::ClaimsVerificationFailed)?;

    let user_email = user_info["email"]
        .as_str()
        .ok_or(AuthError::MissingEmail)?
        .to_string();
    let provider_id = user_info["id"]
        .as_u64()
        .ok_or(AuthError::MissingIdToken)?
        .to_string();

    let user = sso::get_or_create_sso_user(&app_state, user_email, provider, provider_id).await?;

    let access_token = sso::generate_jwt(user.user_id)?;

    tracing::debug!("Generated access token for user");
    session
        .remove::<PkceCodeVerifier>("pkce_verifier")
        .await
        .map_err(|_| AuthError::SessionError)?;
    session
        .remove::<String>("csrf_token")
        .await
        .map_err(|_| AuthError::SessionError)?;

    Ok(Json(ExchangeResponse { access_token }))
}
