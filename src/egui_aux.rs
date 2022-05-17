use eframe::egui::{Response, RichText, Ui};

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

pub fn mono_button(ui: &mut Ui, text: &str) -> Response {
    ui.button(RichText::new(text).monospace())
}
