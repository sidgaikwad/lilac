use anyhow::anyhow;
use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use openidconnect::{
    core::{CoreResponseType},
    AuthenticationFlow, AuthorizationCode, CsrfToken, Nonce,
    PkceCodeChallenge, Scope, TokenResponse,
};
use serde::{Deserialize, Serialize};

use crate::{auth::error::AuthError, AppState};
use openidconnect::core::CoreClient;
pub async fn login(
    State(app_state): State<AppState>,
    Path(provider): Path<String>,
) -> Result<impl IntoResponse, AuthError> {
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
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    let oidc_cookie = OidcCookie {
        nonce: nonce.secret().to_string(),
        pkce_verifier: pkce_verifier.secret().to_string(),
        csrf_token: csrf_token.secret().to_string(),
        provider: provider,
    };

    let cookie = Cookie::build(("oidc_state", serde_json::to_string(&oidc_cookie).unwrap()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    Ok((
        axum::response::AppendHeaders([("Set-Cookie", cookie.to_string())]),
        Redirect::to(auth_url.as_str()),
    ))
}

#[axum::debug_handler]
pub async fn callback(
    State(app_state): State<AppState>,
    Query(query): Query<CallbackQuery>,
    cookie: CookieJar,
) -> Result<impl IntoResponse, AuthError> {
    let oidc_cookie_str = cookie
        .get("oidc_state")
        .map(|cookie| cookie.value().to_string())
        .ok_or(AuthError::MissingOidcCookie)?;

    let oidc_cookie: OidcCookie =
        serde_json::from_str(&oidc_cookie_str).map_err(|_| AuthError::InvalidOidcCookie)?;

    if query.state != oidc_cookie.csrf_token {
        return Err(AuthError::InvalidCsrfToken);
    }

    let config = app_state
        .oidc_configs
        .get(&oidc_cookie.provider)
        .ok_or(AuthError::ProviderNotFound)?;

    let client = CoreClient::from_provider_metadata(
        config.provider_metadata.clone(),
        config.client_id.clone(),
        config.client_secret.clone(),
    )
    .set_redirect_uri(config.redirect_uri.clone());

    let token_response = client
        .exchange_code(AuthorizationCode::new(query.code))
        .map_err(|e| {
            tracing::error!("Failed to create code exchange request: {}", e);
            AuthError::CodeExchangeFailed
        })?
        .set_pkce_verifier(openidconnect::PkceCodeVerifier::new(
            oidc_cookie.pkce_verifier,
        ))
        .request_async(&app_state.http_client)
        .await
        .map_err(|e| {
            tracing::error!("Failed to exchange code: {}", e);
            AuthError::CodeExchangeFailed
        })?;

    let id_token = token_response
        .id_token()
        .ok_or_else(|| anyhow!("Server did not return an ID token"))
        .map_err(|_| AuthError::MissingIdToken)?;

    let claims = id_token
        .claims(
            &client.id_token_verifier(),
            &Nonce::new(oidc_cookie.nonce),
        )
        .map_err(|e| {
            tracing::error!("Failed to verify claims: {}", e);
            AuthError::ClaimsVerificationFailed
        })?;
        
    // TO-DO: Fix my god awful token verification -- idk what the hell i did wrong
    // if let Some(expected_access_token_hash) = claims.access_token_hash() {
    //     let id_token_verifier = client.id_token_verifier();
    //     let actual_access_token_hash = AccessTokenHash::from_token(
    //         token_response.access_token(),
    //         id_token.signing_alg().unwrap(),
    //         &id_token.signing_key(&id_token_verifier).unwrap(),
    //     )
    //     .unwrap();
    //     if actual_access_token_hash != *expected_access_token_hash {
    //         return Err(AuthError::InvalidAccessToken);
    //     }
    // }

    let user_email = claims.email().ok_or(AuthError::MissingEmail)?.to_string();

    let user = match app_state.db.get_user_by_email(&user_email).await {
        Ok(user) => user,
        Err(_) => {
            // User doesn't exist, create a new one
            let new_user = app_state
                .db
                .create_oidc_user(&user_email)
                .await
                .map_err(|_| AuthError::UserCreation)?;
            new_user
        }
    };

    let claims = crate::auth::claims::Claims::create(user.user_id);
    let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &crate::auth::keys::KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    let frontend_redirect_url = &app_state
        .oidc_configs
        .get(&oidc_cookie.provider)
        .ok_or(AuthError::ProviderNotFound)?
        .frontend_redirect_url;

    let mut url = url::Url::parse(frontend_redirect_url).map_err(|_| AuthError::InvalidRedirectUri)?;
    url.query_pairs_mut()
        .append_pair("token", &token);

    Ok(Redirect::to(url.as_str()))
}

#[derive(Debug, Deserialize)]
pub struct CallbackQuery {
    code: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OidcCookie {
    nonce: String,
    pkce_verifier: String,
    csrf_token: String,
    provider: String,
}

pub async fn providers(State(app_state): State<AppState>) -> impl IntoResponse {
    let providers = app_state.oidc_configs.keys().cloned().collect::<Vec<_>>();
    axum::Json(providers)
}