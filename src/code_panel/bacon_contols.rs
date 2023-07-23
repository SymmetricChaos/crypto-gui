use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::other::bacon::Bacon;
use egui::{TextEdit, TextStyle};

pub struct BaconFrame {
    code: Bacon,
}

impl Default for BaconFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for BaconFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.subheading("False Text");
        ui.add(TextEdit::multiline(&mut self.code.false_text).font(TextStyle::Monospace));
        ui.add_space(4.0);
        ui.label(format!(
            "Maximum message length: {} characters",
            self.code.false_text.chars().count() / 5
        ));
        ui.add_space(16.0);
        ui.fill_code_columns(12, 4, self.code.chars_codes());
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
