use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
pub struct NudaConfig {
    pub dev: Option<DevConfig>,
}

#[derive(Debug, Deserialize)]
pub struct DevConfig {
    pub port: u16,
}

pub fn load_config() -> Option<NudaConfig> {
    let config_path = Path::new("nuda.config.json");
    let config_str = fs::read_to_string(config_path).ok()?;
    let config: NudaConfig = serde_json::from_str(&config_str).ok()?;
    Some(config)
}
