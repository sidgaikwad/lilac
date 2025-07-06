use std::{collections::HashMap, path::PathBuf};

use axum::extract::FromRef;
use common::{
    aws::{S3Wrapper, STSWrapper},
    database::Database,
    k8s::K8sWrapper,
};
use config::Config;
use openidconnect::{core::CoreProviderMetadata, reqwest, ClientId, ClientSecret, RedirectUrl};
use serde::Deserialize;

pub mod auth;
pub mod routes;

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceConfig {
    pub gateway_name: String,
    pub gateway_namespace: String,
    pub gateway_url: String,
}

#[derive(Clone)]
pub struct OidcConfig {
    pub provider_metadata: CoreProviderMetadata,
    pub client_id: ClientId,
    pub client_secret: Option<ClientSecret>,
    pub redirect_uri: RedirectUrl,
    pub frontend_redirect_url: String,
}

use oauth2::{
    AuthUrl, ClientId as Oauth2ClientId, ClientSecret as Oauth2ClientSecret,
    RedirectUrl as Oauth2RedirectUrl, TokenUrl,
};
use serde::Deserialize;

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
    pub service_config: ServiceConfig,
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

#[derive(Clone, Debug, Deserialize)]
pub struct TlsConfig {
    #[serde(default)]
    pub enabled: bool,
    pub cert_file: PathBuf,
    pub key_file: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum SsoConfig {
    Oidc {
        client_id: String,
        client_secret: String,
        issuer_url: String,
    },
    Oauth2 {
        client_id: String,
        client_secret: String,
        auth_url: String,
        token_url: String,
        user_info_url: String,
    },
}

#[derive(Clone, Debug, Deserialize)]
pub struct LilacConfig {
    pub database_url: String,
    pub tls: Option<TlsConfig>,
    pub http_port: u16,
    pub sso: HashMap<String, SsoConfig>,
    pub secret_key: String,
    pub frontend_url: String,
}

impl LilacConfig {
    pub fn new() -> Option<Self> {
        let config_file_path = std::env::var("LILAC_CONFIG_FILE");
        match config_file_path {
            Ok(path) => Config::builder()
                .add_source(config::File::with_name(&path))
                .add_source(config::Environment::with_prefix("LILAC").separator("__"))
                .build()
                .map_err(|e| println!("{e:?}"))
                .ok()?
                .try_deserialize()
                .map_err(|e| println!("{e:?}"))
                .ok(),
            Err(_) => Config::builder()
                .add_source(config::Environment::with_prefix("LILAC").separator("__"))
                .build()
                .map_err(|e| println!("{e:?}"))
                .ok()?
                .try_deserialize()
                .map_err(|e| println!("{e:?}"))
                .ok(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{LilacConfig, SsoConfig};

    #[test]
    pub fn load_lilac_config() {
        std::env::set_var("LILAC__DATABASE_URL", "db_url");
        std::env::set_var("LILAC__HTTP_PORT", "8080");
        std::env::set_var("LILAC__SECRET_KEY", "secret");
        std::env::set_var("LILAC__FRONTEND_URL", "frontend_url");
        std::env::set_var("LILAC__TLS__CERT_FILE", "./cert.pem");
        std::env::set_var("LILAC__TLS__KEY_FILE", "./key.pem");
        std::env::set_var("LILAC__SSO__GITHUB__CLIENT_ID", "github_id");
        std::env::set_var("LILAC__SSO__GITHUB__CLIENT_SECRET", "github_secret");
        std::env::set_var("LILAC__SSO__GITHUB__AUTH_URL", "github_auth_url");
        std::env::set_var("LILAC__SSO__GITHUB__TOKEN_URL", "github_token_url");
        std::env::set_var("LILAC__SSO__GITHUB__USER_INFO_URL", "github_user_info_url");
        std::env::set_var("LILAC__SSO__GOOGLE__CLIENT_ID", "google_id");
        std::env::set_var("LILAC__SSO__GOOGLE__CLIENT_SECRET", "google_secret");
        std::env::set_var("LILAC__SSO__GOOGLE__ISSUER_URL", "google_issuer_url");

        let config = LilacConfig::new().unwrap();

        assert_eq!(config.database_url, "db_url");
        assert_eq!(config.http_port, 8080);
        assert_eq!(config.secret_key, "secret");

        let tls = config.tls.unwrap();
        assert_eq!(tls.enabled, false);
        assert_eq!(tls.cert_file.to_str().unwrap(), "./cert.pem");
        assert_eq!(tls.key_file.to_str().unwrap(), "./key.pem");

        assert_eq!(config.sso.len(), 2);
        assert!(config.sso.contains_key("google"));
        assert!(config.sso.contains_key("github"));

        assert!(matches!(config.sso["google"], SsoConfig::Oidc { .. }));
        assert!(matches!(config.sso["github"], SsoConfig::Oauth2 { .. }));
    }
}
