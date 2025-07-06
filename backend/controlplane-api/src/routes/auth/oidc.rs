use super::sso;
use crate::{auth::error::AuthError, AppState};
use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    Json,
};
use openidconnect::{
    core::{CoreClient, CoreResponseType},
    AuthenticationFlow, AuthorizationCode, CsrfToken, Nonce, PkceCodeChallenge, PkceCodeVerifier,
    TokenResponse,
};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    authorization_url: String,
}

pub async fn login(
    State(app_state): State<AppState>,
    Path(provider): Path<String>,
    session: Session,
) -> Result<Json<LoginResponse>, AuthError> {
    let config = app_state
        .oidc_configs
        .get(&provider)
        .ok_or(AuthError::ProviderNotFound)?;

    let client = CoreClient::from_provider_metadata(
        config.provider_metadata.clone(),
        config.client_id.clone(),
        config.client_secret.clone(),
    )
    .set_redirect_uri(config.redirect_uri.clone());

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token, nonce) = client
        .authorize_url(
            AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(openidconnect::Scope::new("email".to_string()))
        .add_scope(openidconnect::Scope::new("profile".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();
    tracing::debug!("Generated authorization URL: {}", auth_url);
    session
        .insert("pkce_verifier", pkce_verifier.secret().clone())
        .await
        .map_err(|_| AuthError::SessionError)?;
    session
        .insert("nonce", nonce.secret().clone())
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
    tracing::debug!("Retrieving nonce and csrf token from session");
    let nonce: String = session
        .get("nonce")
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
        .oidc_configs
        .get(&provider)
        .ok_or(AuthError::ProviderNotFound)?;

    let client = CoreClient::from_provider_metadata(
        config.provider_metadata.clone(),
        config.client_id.clone(),
        config.client_secret.clone(),
    )
    .set_redirect_uri(config.redirect_uri.clone());

    tracing::debug!("Exchanging code for token");
    let token_response = client
        .exchange_code(AuthorizationCode::new(payload.code))
        .map_err(|_| AuthError::CodeExchangeFailed)?
        .set_pkce_verifier(pkce_verifier)
        .request_async(&app_state.http_client)
        .await
        .map_err(|_| AuthError::CodeExchangeFailed)?;

    tracing::debug!("Verifying ID token");
    let id_token = token_response
        .id_token()
        .ok_or_else(|| anyhow!("Server did not return an ID token"))
        .map_err(|_| AuthError::MissingIdToken)?;

    tracing::debug!("Verifying claims in ID token");
    let claims = id_token
        .claims(&client.id_token_verifier(), &Nonce::new(nonce))
        .map_err(|_| AuthError::ClaimsVerificationFailed)?;

    let user_email = claims.email().ok_or(AuthError::MissingEmail)?.to_string();
    let sso_provider_id = claims.subject().to_string();

    let user =
        sso::get_or_create_sso_user(&app_state, user_email, provider, sso_provider_id).await?;

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
    session
        .remove::<String>("nonce")
        .await
        .map_err(|_| AuthError::SessionError)?;

    Ok(Json(ExchangeResponse { access_token }))
}
