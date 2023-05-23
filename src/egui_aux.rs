use eframe::egui::RichText;
use egui::Color32;

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
