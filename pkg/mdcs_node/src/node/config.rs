use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::net::IpAddr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    host: IpAddr,
    port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginConfig {
    pub description: String,
    pub command: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub plugin: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub network: NetworkConfig,
    pub plugins: HashMap<String, PluginConfig>,
    pub devices: Vec<DeviceConfig>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Config, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;

        Ok(config)
    }
}
