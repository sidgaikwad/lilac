use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

// Represents the user-specific configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserConfig {
    pub api_endpoint: String,
    pub api_key: Option<String>,
}

// Represents the agent-specific configuration.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentConfig {
    pub api_endpoint: String,
    pub cluster_api_key: String,
    pub node_id: Uuid,
    pub private_registry: Option<PrivateRegistryConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrivateRegistryConfig {
    pub registry_url: String,
    pub username: String,
    pub secret: String,
}

pub fn load_user_config() -> anyhow::Result<UserConfig> {
    let config_path = get_config_path("config.toml")?;

    // Prioritize environment variables
    if let Ok(api_endpoint) = env::var("LILAC_API_ENDPOINT") {
        let config = UserConfig {
            api_endpoint,
            api_key: env::var("LILAC_USER_API_KEY").ok(),
        };
        // Write to file if env vars are used, to persist the config
        let toml_string = toml::to_string(&config)?;
        fs::create_dir_all(config_path.parent().unwrap())?;
        fs::write(&config_path, toml_string)?;
        return Ok(config);
    }

    // Fallback to config file
    if !config_path.exists() {
        let config = UserConfig {
            api_endpoint: "http://localhost:8080".to_string(),
            ..Default::default()
        };
        let toml_string = toml::to_string(&config)?;
        fs::create_dir_all(config_path.parent().unwrap())?;
        fs::write(&config_path, toml_string)?;
        println!("Created new user config file at: {:?}", config_path);
        return Ok(config);
    }

    let content = fs::read_to_string(&config_path)?;
    Ok(toml::from_str(&content)?)
}

pub fn load_agent_config() -> anyhow::Result<AgentConfig> {
    let config_path = get_config_path("agent.toml")?;

    // Prioritize environment variables
    if let Ok(api_endpoint) = env::var("LILAC_API_ENDPOINT") {
        let config = AgentConfig {
            api_endpoint,
            cluster_api_key: env::var("LILAC_CLUSTER_API_KEY")?,
            node_id: env::var("LILAC_NODE_ID")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or_else(Uuid::new_v4),
            private_registry: if let Ok(registry_url) = env::var("LILAC_PRIVATE_REGISTRY_URL") {
                Some(PrivateRegistryConfig {
                    registry_url,
                    username: env::var("LILAC_PRIVATE_REGISTRY_USERNAME")?,
                    secret: env::var("LILAC_PRIVATE_REGISTRY_PASSWORD")?,
                })
            } else {
                None
            },
        };
        // Write to file if env vars are used, to persist the config
        let toml_string = toml::to_string(&config)?;
        fs::create_dir_all(config_path.parent().unwrap())?;
        fs::write(&config_path, toml_string)?;
        return Ok(config);
    }

    // Fallback to config file
    if !config_path.exists() {
        let config = AgentConfig {
            api_endpoint: "http://localhost:8080".to_string(),
            cluster_api_key: "".to_string(),
            node_id: Uuid::new_v4(),
            private_registry: None,
        };
        let toml_string = toml::to_string(&config)?;
        fs::create_dir_all(config_path.parent().unwrap())?;
        fs::write(&config_path, toml_string)?;
        println!("Created new agent config file at: {:?}", config_path);
        return Ok(config);
    }

    let content = fs::read_to_string(&config_path)?;
    Ok(toml::from_str(&content)?)
}

pub fn get_config_path(file_name: &str) -> anyhow::Result<PathBuf> {
    let home_dir =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let config_dir = home_dir.join(".lilac");
    Ok(config_dir.join(file_name))
}