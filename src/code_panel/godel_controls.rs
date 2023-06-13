use crate::ui_elements::fill_code_columns;

use super::CodeFrame;

use codes::{mathematical::godel::Godel, traits::IOMode};
use egui::TextEdit;

pub struct GodelFrame {
    code: Godel,
}

impl Default for GodelFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for GodelFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.selectable_value(&mut self.code.mode, IOMode::Letter, "Letter");
        ui.selectable_value(&mut self.code.mode, IOMode::Word, "Word");
        match self.code.mode {
            IOMode::Letter => {
                ui.label("Alphabetical Mode: Provide an alphabet. Codes will be assigned to each character of the alphabet in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui
                    .add(TextEdit::singleline(&mut self.code.maps.alphabet))
                    .changed()
                {
                    self.code.set_letter_map();
                };
                fill_code_columns(16, 5, ui, Box::new(self.code.maps.codes_chars()));
            }
            IOMode::Word => {
                ui.label("Word Mode: Provide any number of words or phrases separated by commas. Codes will be assigned to each word or phrase in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui
                    .add(TextEdit::singleline(&mut self.code.maps.words_string))
                    .changed()
                {
                    self.code.set_word_map();
                };
                fill_code_columns(16, 5, ui, Box::new(self.code.maps.codes_words()));
            }
            IOMode::Integer => {
                ui.label("<<<ERROR INTEGER MODE IS NOT DEFINED FOR GODEL CODE>>>");
            }
        }
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
