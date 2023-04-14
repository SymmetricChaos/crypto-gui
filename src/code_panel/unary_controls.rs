use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::{unary::UnaryMode, UnaryCode};
use eframe::egui::TextEdit;

impl ViewableCode for UnaryCode {}

impl View for UnaryCode {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.selectable_value(&mut self.mode, UnaryMode::Letter, "Letter");
        ui.selectable_value(&mut self.mode, UnaryMode::Word, "Word");
        match self.mode {
            UnaryMode::Letter => {
                ui.label("Alphabetical Mode: Provide an alphabet. Codes will be assigned to each character of the alphabet in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui
                    .add(TextEdit::singleline(&mut self.code.alphabet))
                    .changed()
                {
                    self.set_letter_map();
                };
                fill_code_columns(16, 5, ui, Box::new(self.code.chars_codes()));
            }
            UnaryMode::Word => {
                ui.label("Word Mode: Provide any number of words or phrases separated by commas. Codes will be assigned to each word or phrase in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui
                    .add(TextEdit::singleline(&mut self.code.words_string))
                    .changed()
                {
                    self.set_word_map();
                };
                fill_code_columns(16, 5, ui, Box::new(self.code.words_codes()));
            }
        }
    }
}
