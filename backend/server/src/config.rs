use secrecy::SecretString;
use serde::Deserialize;
use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::filter::Directive;
use std::{collections::HashMap, path::PathBuf};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TlsConfig {
    #[serde(default)]
    pub enabled: bool,
    pub cert_file: PathBuf,
    pub key_file: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
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

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum LogFormat {
    #[default]
    Pretty,
    Json,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl From<&LogLevel> for Directive {
    fn from(value: &LogLevel) -> Self {
        let level = match value {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        };
        LevelFilter::from_level(level).into()
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LilacConfig {
    pub database_url: SecretString,
    pub tls: Option<TlsConfig>,
    pub http_port: u16,
    pub sso: Option<HashMap<String, SsoConfig>>,
    pub secret_key: SecretString,
    pub frontend_url: String,
    #[serde(default)]
    pub log_format: LogFormat,
    #[serde(default)]
    pub log_level: LogLevel,
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