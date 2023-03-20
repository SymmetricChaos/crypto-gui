use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::Godel;
use eframe::egui::TextEdit;

impl ViewableCode for Godel {}

impl View for Godel {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.add(TextEdit::singleline(self.control_alphabet()));
        fill_code_columns(20, 3, ui, Box::new(self.chars_codes()));
    }
}
