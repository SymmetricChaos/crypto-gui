use eframe::egui::TextEdit;
use super::{View, generic_components::fill_code_columns};
use crate::codes::UnaryCode;


impl View for UnaryCode {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.add(TextEdit::singleline(&mut self.alphabet));
        fill_code_columns(20, 3, ui, Box::new(self.chars_codes()));
    }
}