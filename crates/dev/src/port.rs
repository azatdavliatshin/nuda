use shared::config::load_config;

// TODO: add logging for port source
pub fn dev_port() -> u16 {
    if let Ok(port_str) = std::env::var("NUDA_DEV_PORT") {
        if let Ok(port) = port_str.parse::<u16>() {
            return port;
        }
    }

    if let Some(cfg) = load_config() {
        if let Some(dev) = cfg.dev {
            return dev.port;
        }
    }

    3000
}
