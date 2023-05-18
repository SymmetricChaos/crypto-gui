use eframe::egui::{Response, RichText, Ui};
use egui::Color32;

pub fn subheading(text: &str) -> RichText {
    RichText::new(text).size(16.0)
}

pub fn mono(ui: &mut Ui, text: &str, size: Option<f32>) -> Response {
    match size {
        Some(n) => ui.label(RichText::new(text).monospace().size(n)),
        None => ui.label(RichText::new(text).monospace()),
    }
}

pub fn mono_strong(ui: &mut Ui, text: &str, size: Option<f32>) -> Response {
    match size {
        Some(n) => ui.label(RichText::new(text).monospace().strong().size(n)),
        None => ui.label(RichText::new(text).monospace().strong()),
    }
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
