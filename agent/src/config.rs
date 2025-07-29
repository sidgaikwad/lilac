use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub api_endpoint: String,
    pub api_key: Option<String>,
    pub container_registry_url: String,
    pub base_image: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_endpoint: "http://localhost:8080".to_string(),
            api_key: None,
            container_registry_url: "docker.io/lilac-user".to_string(),
            base_image: "python:3.10-slim-bullseye".to_string(),
        }
    }
}


pub fn load() -> anyhow::Result<Config> {
    // Prioritize environment variables
    if let (Ok(api_endpoint), Ok(registry_url), Ok(base_image)) = (
        env::var("LILAC_API_ENDPOINT"),
        env::var("LILAC_CONTAINER_REGISTRY_URL"),
        env::var("LILAC_BASE_IMAGE"),
    ) {
        return Ok(Config {
            api_endpoint,
            api_key: env::var("LILAC_API_KEY").ok(),
            container_registry_url: registry_url,
            base_image,
        });
    }

    // Fallback to config file
    let config_path = get_config_path()?;
    if !config_path.exists() {
        return Ok(Config::default());
    }

    let content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&content)?;

    Ok(config)
}

pub fn get_config_path() -> anyhow::Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let config_dir = home_dir.join(".lilac");
    Ok(config_dir.join("config.toml"))
}