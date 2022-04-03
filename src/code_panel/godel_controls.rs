use eframe::egui::TextEdit;
use super::{View, generic_components::fill_code_columns};
use crate::codes::Godel;


impl View for Godel {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.add(TextEdit::singleline(self.control_alphabet()));
        fill_code_columns(20, 3, ui, Box::new(self.chars_codes()));
    }
}
