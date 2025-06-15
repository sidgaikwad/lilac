use std::collections::HashMap;

use axum::extract::FromRef;
use common::{
    aws::{S3Wrapper, STSWrapper},
    database::Database,
};
use openidconnect::{
    core::{CoreProviderMetadata},
    reqwest, ClientId, ClientSecret, RedirectUrl,
    k8s::K8sWrapper,
};

pub mod auth;
pub mod routes;
mod tenants;

#[derive(Clone)]
pub struct OidcConfig {
    pub provider_metadata: CoreProviderMetadata,
    pub client_id: ClientId,
    pub client_secret: Option<ClientSecret>,
    pub redirect_uri: RedirectUrl,
    pub frontend_redirect_url: String,
}

use oauth2::{AuthUrl, ClientId as Oauth2ClientId, ClientSecret as Oauth2ClientSecret, RedirectUrl as Oauth2RedirectUrl, TokenUrl};

#[derive(Clone)]
pub struct Oauth2Config {
    pub client_id: Oauth2ClientId,
    pub client_secret: Oauth2ClientSecret,
    pub auth_url: AuthUrl,
    pub token_url: TokenUrl,
    pub redirect_url: Oauth2RedirectUrl,
    pub user_info_url: String,
    pub frontend_redirect_url: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub s3: S3Wrapper,
    pub sts: STSWrapper,
    pub oidc_configs: HashMap<String, OidcConfig>,
    pub oauth2_configs: HashMap<String, Oauth2Config>,
    pub http_client: reqwest::Client,
    pub k8s: K8sWrapper,
}

impl FromRef<AppState> for Database {
    fn from_ref(app_state: &AppState) -> Database {
        app_state.db.clone()
    }
}

impl FromRef<AppState> for S3Wrapper {
    fn from_ref(app_state: &AppState) -> S3Wrapper {
        app_state.s3.clone()
    }
}

impl FromRef<AppState> for STSWrapper {
    fn from_ref(app_state: &AppState) -> STSWrapper {
        app_state.sts.clone()
    }
}

impl FromRef<AppState> for HashMap<String, OidcConfig> {
    fn from_ref(app_state: &AppState) -> HashMap<String, OidcConfig> {
        app_state.oidc_configs.clone()
    }
}

impl FromRef<AppState> for K8sWrapper {
    fn from_ref(app_state: &AppState) -> K8sWrapper {
        app_state.k8s.clone()
    }
}
