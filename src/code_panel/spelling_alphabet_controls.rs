use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::SpellingAlphabet;

impl ViewableCode for SpellingAlphabet {}

impl View for SpellingAlphabet {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        fill_code_columns(9, 4, ui, Box::new(self.chars_codes()));
    }
}
