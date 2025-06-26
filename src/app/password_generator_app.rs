use eframe::{App, Frame};
use egui::Context;
use crate::config::FileConfigManager;
use crate::controllers::PasswordController;
use crate::clipboard::ProductionClipboardManager;
use crate::generators::Sha256PasswordGenerator;

pub struct PasswordGeneratorApp {
    controller: PasswordController,
    master_password: String,
    service_name: String,
    clipboard_message: String,
    clipboard_message_time: std::time::Instant,
}

impl PasswordGeneratorApp {
    pub fn new() -> Self {
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
            clipboard_message: String::new(),
            clipboard_message_time: std::time::Instant::now(),
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
                            if self.controller.copy_password_to_clipboard(
                                &self.master_password,
                                &self.service_name,
                            ) {
                                self.clipboard_message = "¡Copiado!".to_string();
                                self.clipboard_message_time = std::time::Instant::now();
                            }
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

impl App for PasswordGeneratorApp {
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

                            if !self.clipboard_message.is_empty() &&
                                self.clipboard_message_time.elapsed().as_secs() < 2 {
                                ui.add_space(5.0);
                                ui.label(
                                    egui::RichText::new(&self.clipboard_message)
                                        .color(egui::Color32::GREEN)
                                        .size(12.0)
                                );
                            } else if self.clipboard_message_time.elapsed().as_secs() >= 2 {
                                self.clipboard_message.clear();
                            }
                        })
                })
            });
    }
}