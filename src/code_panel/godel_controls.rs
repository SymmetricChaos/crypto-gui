use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::{Godel, IOMode};
use eframe::egui::TextEdit;

impl ViewableCode for Godel {}

impl View for Godel {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.selectable_value(&mut self.mode, IOMode::Letter, "Letter");
        ui.selectable_value(&mut self.mode, IOMode::Word, "Word");
        match self.mode {
            IOMode::Letter => {
                ui.label("Alphabetical Mode: Provide an alphabet. Codes will be assigned to each character of the alphabet in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui
                    .add(TextEdit::singleline(&mut self.maps.alphabet))
                    .changed()
                {
                    self.set_letter_map();
                };
                fill_code_columns(16, 5, ui, Box::new(self.maps.codes_chars()));
            }
            IOMode::Word => {
                ui.label("Word Mode: Provide any number of words or phrases separated by commas. Codes will be assigned to each word or phrase in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui
                    .add(TextEdit::singleline(&mut self.maps.words_string))
                    .changed()
                {
                    self.set_word_map();
                };
                fill_code_columns(16, 5, ui, Box::new(self.maps.codes_words()));
            }
            IOMode::Integer => {
                ui.label("<<<ERROR INTEGER MODE IS NOT DEFINED FOR GODEL CODE>>>");
            }
        }
    }
}
