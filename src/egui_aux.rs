use eframe::egui::RichText;
use egui::Color32;

pub fn subheading(text: &str) -> RichText {
    RichText::new(text).size(16.0)
}

pub fn mono<T: ToString>(text: T) -> RichText {
    RichText::from(text.to_string()).monospace()
}

pub fn mono_strong<T: ToString>(text: T) -> RichText {
    RichText::from(text.to_string()).monospace().strong()
}

pub fn mono_text(text: &str, size: Option<f32>) -> RichText {
    match size {
        Some(n) => RichText::new(text).monospace().size(n),
        None => RichText::new(text).monospace(),
    }
}

pub fn mono_strong_text(text: &str, size: Option<f32>) -> RichText {
    match size {
        Some(n) => RichText::new(text).monospace().strong().size(n),
        None => RichText::new(text).monospace().strong(),
    }
}

pub fn error_text(text: &str) -> RichText {
    RichText::new(text)
        .color(Color32::RED)
        .background_color(Color32::BLACK)
        .monospace()
}
