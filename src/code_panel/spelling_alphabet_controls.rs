use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::{spelling_alphabet::SpellingAlphabetMode, SpellingAlphabet};

impl ViewableCode for SpellingAlphabet {}

impl View for SpellingAlphabet {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mode, SpellingAlphabetMode::Nato, "NATO");
            ui.selectable_value(&mut self.mode, SpellingAlphabetMode::Ccb, "CCB");
        });
        fill_code_columns(9, 4, ui, Box::new(self.chars_codes()));
    }
}
