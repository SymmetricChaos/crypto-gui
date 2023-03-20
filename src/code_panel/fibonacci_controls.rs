use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::FibonacciCode;
use eframe::egui::TextEdit;

impl ViewableCode for FibonacciCode {}

impl View for FibonacciCode {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.add(TextEdit::singleline(&mut self.alphabet));
        fill_code_columns(32, 4, ui, Box::new(self.chars_codes()));
    }
}
