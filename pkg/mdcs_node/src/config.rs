use std::fs;
use std::error::Error;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginConfig {
    pub description: String,
    pub command: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub plugin: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeConfig {
    pub plugins: HashMap<String, PluginConfig>,
    pub devices: Vec<DeviceConfig>
}

impl NodeConfig {
    pub fn from_file(path: &str) -> Result<NodeConfig, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        let config: NodeConfig = serde_yaml::from_str(&content)?;

        Ok(config)
    }
}
