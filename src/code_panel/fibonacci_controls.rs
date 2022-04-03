use eframe::egui::TextEdit;
use super::{View, generic_components::fill_code_columns};
use crate::codes::FibonacciCode;


impl View for FibonacciCode {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.add(TextEdit::singleline(&mut self.alphabet));
        fill_code_columns(32, 4, ui, Box::new(self.chars_codes()));
    }
}
