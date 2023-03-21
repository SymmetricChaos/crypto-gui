use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::MorseAmerican;

impl ViewableCode for MorseAmerican {}

impl View for MorseAmerican {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        fill_code_columns(20, 3, ui, Box::new(self.chars_codes()))
    }
}
