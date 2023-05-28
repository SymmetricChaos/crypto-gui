use eframe::egui::RichText;
use egui::{Color32, Ui};

pub fn subheading<T: ToString>(text: T) -> RichText {
    RichText::from(text.to_string()).size(16.0)
}

pub fn mono<T: ToString>(text: T) -> RichText {
    RichText::from(text.to_string()).monospace()
}

pub fn mono_strong<T: ToString>(text: T) -> RichText {
    RichText::from(text.to_string()).monospace().strong()
}

pub fn error_text<T: ToString>(text: T) -> RichText {
    RichText::from(text.to_string())
        .color(Color32::RED)
        .background_color(Color32::BLACK)
        .monospace()
}

pub fn text_manip_menu(ui: &mut Ui, text: &mut String) {
    ui.menu_button("+", |ui| {
        if ui.button("Remove Whitespace").clicked() {
            *text = text.split_whitespace().collect();
        }
        if ui.button("UPPERCASE").clicked() {
            *text = text.to_uppercase();
        }
        if ui.button("lowercase").clicked() {
            *text = text.to_lowercase();
        }
    });
}
