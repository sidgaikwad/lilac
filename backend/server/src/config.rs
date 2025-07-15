use secrecy::SecretString;
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

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
    pub database_url: SecretString,
    pub tls: Option<TlsConfig>,
    pub http_port: u16,
    pub sso: Option<HashMap<String, SsoConfig>>,
    pub secret_key: SecretString,
    pub frontend_url: String,
}

impl LilacConfig {
    pub fn new() -> Option<Self> {
        let config_file_path = std::env::var("LILAC_CONFIG_FILE");
        match config_file_path {
            Ok(path) => config::Config::builder()
                .add_source(config::File::with_name(&path))
                .add_source(config::Environment::with_prefix("LILAC").separator("__"))
                .build()
                .map_err(|e| println!("{e:?}"))
                .ok()?
                .try_deserialize()
                .map_err(|e| println!("{e:?}"))
                .ok(),
            Err(_) => config::Config::builder()
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