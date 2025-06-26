use crate::config::{config_types::Config, traits::ConfigManager};
use std::fs;

pub struct FileConfigManager {
    config_file: String,
}

impl FileConfigManager {
    pub fn new(config_file: &str) -> Self {
        Self {
            config_file: config_file.to_string(),
        }
    }
}

impl ConfigManager for FileConfigManager {
    fn load(&self) -> Config {
        match fs::read_to_string(&self.config_file) {
            Ok(content) => {
                serde_json::from_str(&content).unwrap_or_default()
            }
            Err(_) => Config::default(),
        }
    }

    fn save(&self, config: &Config) {
        if let Ok(json) = serde_json::to_string_pretty(config) {
            let _ = fs::write(&self.config_file, json);
        }
    }
}