
use eframe::{Frame, egui};
use egui::Context;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use std::fs;

fn main() -> Result<(), eframe::Error>{

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_resizable(false),
        ..Default::default()
    };
    
    eframe::run_native(
        "Password Generator",
        options,
        Box::new(|cc| Ok(Box::new(PasswordGeneratorApp::new())))
    )
}


#[derive(Serialize, Deserialize, Default)]
struct Config {
    show_password: bool,
}


trait PasswordGenerator {
    fn generate(&self, secret: &str, service: &str) -> String;
}


struct Sha256PasswordGenerator;


impl PasswordGenerator for Sha256PasswordGenerator {
    
    fn generate(&self, secret: &str, service: &str) -> String {
        let data = format!("{}-{}", secret, service);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)[..16].to_string()
    }
}

trait ConfigManager {
    fn load(&self) -> Config;
    fn save(&self, config: &Config);
}

struct FileConfigManager {
    config_file: String,
}

impl FileConfigManager {
    fn new(config_file: &str) -> Self {
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

trait ClipboardManager {
    fn copy_to_clipboard(&self, text: &str);
}

struct ProductionClipboardManager;

impl ClipboardManager for ProductionClipboardManager {
    fn copy_to_clipboard(&self, text: &str) {
        match arboard::Clipboard::new() {
            Ok(mut clipboard) => {
                match clipboard.set_text(text) {
                    Ok(_) => {
                        println!("✓ Contraseña copiada al portapapeles");
                    }
                    Err(e) => {
                        eprintln!("Error copiando al portapapeles: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error accediendo al portapapeles: {}", e);
            }
        }
    }
}


// Implementacion Mock
struct MockClipboardManager;

impl ClipboardManager for MockClipboardManager {
    fn copy_to_clipboard(&self, text: &str) {
        println!("Copiado al portapapeles: {}", text);
    }
}


struct PasswordController {
    password_generator: Box<dyn PasswordGenerator>,
    config_manager: Box<dyn ConfigManager>,
    clipboard_manager: Box<dyn ClipboardManager>,
    config: Config,
}


impl PasswordController {
    fn new(
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
    
    fn generate_password(&self, secret: &str, service: &str) -> String {
        if secret.is_empty() || service.is_empty() {
            return String::new();
        }
        self.password_generator.generate(secret, service)
    }
    
    fn get_visible_password(&self, secret: &str, service: &str) -> String {
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
    
    fn toggle_password_visibility(&mut self) {
        self.config.show_password = !self.config.show_password;
        self.config_manager.save(&self.config);
    }

    fn copy_password_to_clipboard(&self, secret: &str, service: &str) {
        let password = self.generate_password(secret, service);
        if !password.is_empty() {
            self.clipboard_manager.copy_to_clipboard(&password);
        }
    }
    
    fn is_password_visible(&self) -> bool {
        self.config.show_password
    }
}

struct PasswordGeneratorApp {
    controller: PasswordController,
    master_password: String,
    service_name: String,
}

impl PasswordGeneratorApp {
    fn new() -> Self {
        let password_generator = Box::new(Sha256PasswordGenerator);
        let config_manager = Box::new(FileConfigManager::new("config.json"));
        let clipboard_manager = Box::new(ProductionClipboardManager);

        let controller = PasswordController::new(
            password_generator,
            config_manager,
            clipboard_manager,
        );

        Self {
            controller,
            master_password: String::new(),
            service_name: String::new(),
        }
    }

    fn render_password_field(&mut self, ui: &mut egui::Ui) {
        let visible_password = self.controller.get_visible_password(
            &self.master_password,
            &self.service_name,
        );

        let response = egui::Frame::NONE
            .fill(egui::Color32::WHITE)
            .corner_radius(egui::CornerRadius::same(4))
            .inner_margin(egui::Margin::ZERO)
            .show(ui, |ui| {
                ui.allocate_ui_with_layout(
                    egui::Vec2::new(ui.available_width(), 30.0),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        let label_response = ui.allocate_ui_with_layout(
                            egui::Vec2::new(ui.available_width() - 30.0, 30.0),
                            egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                            |ui| {
                                ui.label(
                                    egui::RichText::new(&visible_password)
                                        .color(egui::Color32::BLACK)
                                        .size(16.0)
                                )
                            }
                        );

                        if label_response.response.hovered() {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        }

                        if label_response.response.clicked() {
                            self.controller.copy_password_to_clipboard(
                                &self.master_password,
                                &self.service_name,
                            );
                        }

                        let button_size = 30.0;
                        let icon = if self.controller.is_password_visible() { "👁" } else { "🙈" };

                        let button_response = ui.add_sized(
                            [button_size, button_size],
                            egui::Button::new(
                                egui::RichText::new(icon)
                                    .color(egui::Color32::BLACK)
                                    .size(16.0)
                            )
                                .fill(egui::Color32::TRANSPARENT)
                                .stroke(egui::Stroke::NONE)
                                .rounding(egui::Rounding::ZERO)
                        );

                        if button_response.hovered() {
                            ui.painter().rect_filled(
                                button_response.rect,
                                egui::Rounding::ZERO,
                                egui::Color32::from_gray(200)
                            );
                        }

                        if button_response.clicked() {
                            self.controller.toggle_password_visibility();
                        }
                    }
                );
            });
    }
}

impl eframe::App for PasswordGeneratorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let visuals = egui::Visuals {
            window_fill: egui::Color32::from_rgb(60, 120, 120),
            ..egui::Visuals::dark()
        };
        ctx.set_visuals(visuals);

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(egui::Color32::from_rgb(145, 55, 65)))
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.add_space(50.0);

                    egui::Frame::NONE
                        .fill(egui::Color32::from_rgb(60, 70, 80))
                        .stroke(egui::Stroke::new(
                            1.0,
                            egui::Color32::from_rgb(100, 110, 120),
                        ))
                        .corner_radius(egui::CornerRadius::same(10))
                        .inner_margin(egui::Margin::same(20))
                        .show(ui, |ui| {
                            ui.set_width(300.0);

                            ui.add(
                                egui::TextEdit::singleline(&mut self.master_password)
                                    .password(true)
                                    .hint_text("Contraseña maestra")
                                    .desired_width(f32::INFINITY)
                                    .min_size(egui::Vec2::new(0.0, 36.0))
                                    .font(egui::FontId::proportional(16.0))
                                    .vertical_align(egui::Align::Center)
                                    .horizontal_align(egui::Align::Center),
                            );

                            ui.add_space(10.0);

                            ui.add(
                                egui::TextEdit::singleline(&mut self.service_name)
                                    .hint_text("Nombre del servicio")
                                    .desired_width(f32::INFINITY)
                                    .min_size(egui::Vec2::new(0.0, 36.0))
                                    .font(egui::FontId::proportional(16.0))
                                    .vertical_align(egui::Align::Center)
                                    .horizontal_align(egui::Align::Center),
                            );

                            ui.add_space(15.0);

                            self.render_password_field(ui);
                        })
                })
            });
    }
}