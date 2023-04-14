use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::UnaryCode;
use eframe::egui::TextEdit;

impl ViewableCode for UnaryCode {}

impl View for UnaryCode {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        if ui.add(TextEdit::singleline(&mut self.alphabet)).changed() {
            self.set_map();
        };
        fill_code_columns(20, 3, ui, Box::new(self.chars_codes()));
    }
}
