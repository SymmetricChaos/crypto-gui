use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::MorseITU;

impl ViewableCode for MorseITU {}

impl View for MorseITU {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        fill_code_columns(20, 3, ui, Box::new(self.chars_codes()))
    }
}
