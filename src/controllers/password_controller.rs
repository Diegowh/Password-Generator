use crate::clipboard::ClipboardManager;
use crate::config::{Config, ConfigManager};
use crate::generators::PasswordGenerator;

pub struct PasswordController {
    password_generator: Box<dyn PasswordGenerator>,
    config_manager: Box<dyn ConfigManager>,
    clipboard_manager: Box<dyn ClipboardManager>,
    config: Config,
}


impl PasswordController {
    pub fn new(
        password_generator: Box<dyn PasswordGenerator>,
        config_manager: Box<dyn ConfigManager>,
        clipboard_manager: Box<dyn ClipboardManager>,
    ) -> Self {
        let config = config_manager.load();
        Self {
            password_generator,
            config_manager,
            clipboard_manager,
            config,
        }
    }

    pub fn generate_password(&self, secret: &str, service: &str) -> String {
        if secret.is_empty() || service.is_empty() {
            return String::new();
        }
        self.password_generator.generate(secret, service)
    }

    pub fn get_visible_password(&self, secret: &str, service: &str) -> String {
        let password = self.generate_password(secret, service);
        if password.is_empty() {
            return String::new();
        }

        if self.config.show_password {
            password
        } else {
            "*".repeat(password.len())
        }
    }

    pub fn toggle_password_visibility(&mut self) {
        self.config.show_password = !self.config.show_password;
        self.config_manager.save(&self.config);
    }

    pub fn copy_password_to_clipboard(&self, secret: &str, service: &str) -> bool {
        let password = self.generate_password(secret, service);
        if !password.is_empty() {
            self.clipboard_manager.copy_to_clipboard(&password);
            true
        } else {
            false
        }
    }

    pub fn is_password_visible(&self) -> bool {
        self.config.show_password
    }
}
