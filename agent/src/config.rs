use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub api_endpoint: String,
    pub api_key: Option<String>,
    pub container_registry_url: String,
    pub base_image: String,
    pub node_id: Option<Uuid>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_endpoint: "http://localhost:8080".to_string(),
            api_key: None,
            container_registry_url: "docker.io/lilac-user".to_string(),
            base_image: "python:3.10-slim-bullseye".to_string(),
            node_id: None,
        }
    }
}

pub fn load() -> anyhow::Result<Config> {
    // Prioritize environment variables
    if let Ok(api_endpoint) = env::var("LILAC_API_ENDPOINT") {
        return Ok(Config {
            api_endpoint,
            api_key: env::var("LILAC_API_KEY").ok(),
            container_registry_url: env::var("LILAC_CONTAINER_REGISTRY_URL")
                .unwrap_or_else(|_| "docker.io/lilac-user".to_string()),
            base_image: env::var("LILAC_BASE_IMAGE")
                .unwrap_or_else(|_| "python:3.10-slim-bullseye".to_string()),
            node_id: env::var("LILAC_NODE_ID").ok().and_then(|s| s.parse().ok()),
        });
    }

    // Fallback to config file
    let config_path = get_config_path()?;
    if !config_path.exists() {
        let mut config = Config::default();
        config.node_id = Some(Uuid::new_v4());
        let toml_string = toml::to_string(&config)?;
        fs::create_dir_all(config_path.parent().unwrap())?;
        fs::write(&config_path, toml_string)?;
        println!("Created new config file at: {:?}", config_path);
        return Ok(config);
    }

    let content = fs::read_to_string(&config_path)?;
    let mut config: Config = toml::from_str(&content)?;

    if config.node_id.is_none() {
        config.node_id = Some(Uuid::new_v4());
        let toml_string = toml::to_string(&config)?;
        fs::write(config_path, toml_string)?;
    }

    Ok(config)
}

pub fn get_config_path() -> anyhow::Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let config_dir = home_dir.join(".lilac");
    Ok(config_dir.join("config.toml"))
}