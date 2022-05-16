use super::{generic_components::fill_code_columns, View};
use crate::codes::UnaryCode;
use eframe::egui::TextEdit;

impl View for UnaryCode {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _input: &mut String, _output: &mut String, _errors: &mut String) {
        ui.add(TextEdit::singleline(&mut self.alphabet));
        fill_code_columns(20, 3, ui, Box::new(self.chars_codes()));
    }
}
