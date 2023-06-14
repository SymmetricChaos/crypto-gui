use super::CodeFrame;
use crate::ui_elements::{fill_code_columns, subheading};
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
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.label(subheading("False Text"));
        ui.add(TextEdit::multiline(&mut self.code.false_text).font(TextStyle::Monospace));
        fill_code_columns(12, 4, ui, self.code.chars_codes());
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
