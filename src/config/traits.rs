use crate::config::config_types::Config;

pub trait ConfigManager {
    fn load(&self) -> Config;
    fn save(&self, config: &Config);
}