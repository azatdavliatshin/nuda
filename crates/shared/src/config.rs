use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
pub struct NudaConfig {
    pub name: Option<String>,
    pub dev: Option<DevConfig>,
}

#[derive(Debug, Deserialize)]
pub struct DevConfig {
    #[serde(default = "default_port")]
    pub port: u16,
    pub host: Option<String>,
}

fn default_port() -> u16 {
    3000
}

pub fn load_config() -> Option<NudaConfig> {
    load_config_from(".")
}

pub fn load_config_from(dir: impl AsRef<Path>) -> Option<NudaConfig> {
    let path = dir.as_ref().join("nuda.config.json");
    let config_str = fs::read_to_string(path).ok()?;
    let config: NudaConfig = serde_json::from_str(&config_str).ok()?;
    Some(config)
}

pub fn is_project_root(dir: impl AsRef<Path>) -> bool {
    let path = dir.as_ref().join("nuda.config.json");
    if !path.exists() {
        false
    } else {
        let config = load_config_from(dir);
        config.is_some()
    }
}
