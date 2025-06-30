use std::collections::HashMap;

use axum::extract::FromRef;
use common::{
    aws::{S3Wrapper, STSWrapper},
    database::Database,
};
use openidconnect::{
    core::{CoreProviderMetadata},
    reqwest, ClientId, ClientSecret, RedirectUrl,
};

pub mod auth;
pub mod routes;

#[derive(Clone)]
pub struct OidcConfig {
    pub provider_metadata: CoreProviderMetadata,
    pub client_id: ClientId,
    pub client_secret: Option<ClientSecret>,
    pub redirect_uri: RedirectUrl,
    pub frontend_redirect_url: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub s3: S3Wrapper,
    pub sts: STSWrapper,
    pub oidc_configs: HashMap<String, OidcConfig>,
    pub http_client: reqwest::Client,
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
