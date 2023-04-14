use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::{godel::GodelMode, Godel};
use eframe::egui::TextEdit;

impl ViewableCode for Godel {}

impl View for Godel {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.selectable_value(&mut self.mode, GodelMode::Letter, "Letter");
        ui.selectable_value(&mut self.mode, GodelMode::Word, "Word");
        match self.mode {
            GodelMode::Letter => {
                ui.label("Alphabetical Mode: Provide an alphabet. Codes will be assigned to each character of the alphabet in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui
                    .add(TextEdit::singleline(&mut self.maps.alphabet))
                    .changed()
                {
                    self.set_letter_map();
                };
                fill_code_columns(16, 5, ui, Box::new(self.maps.chars_codes()));
            }
            GodelMode::Word => {
                ui.label("Word Mode: Provide any number of words or phrases separated by commas. Codes will be assigned to each word or phrase in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui
                    .add(TextEdit::singleline(&mut self.maps.words_string))
                    .changed()
                {
                    self.set_word_map();
                };
                fill_code_columns(16, 5, ui, Box::new(self.maps.words_codes()));
            }
        }
    }
}
