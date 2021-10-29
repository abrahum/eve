use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tracing::info;

static CONFIG_FILE_NAME: &'static str = "Eve.toml";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub clients: Vec<ClientConfig>, // yes, eve will support multi-clinet!
    pub debug:   bool,
    pub trace:   bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientConfig {
    pub uid:      i64,
    pub password: String,
    // libob config here
}

impl Config {
    pub fn load_or_new() -> Self {
        use std::fs;
        use std::path::PathBuf;

        let path = PathBuf::from_str(CONFIG_FILE_NAME).unwrap();
        if path.exists() {
            // load
            info!("Loading Config from {}", CONFIG_FILE_NAME);
            let file_str = fs::read_to_string(&path).unwrap();
            let config: Self = toml::from_str(&file_str).unwrap();
            info!("Using Config from config file");
            config
        } else {
            // new
            info!("{} not exists, creating default config", CONFIG_FILE_NAME);
            let config = Self::default();
            let config_toml = toml::to_string(&config).unwrap();
            fs::write(&path, config_toml).unwrap();
            info!("Using Default config");
            config
        }
    }
}
