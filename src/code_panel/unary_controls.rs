use codes::mathematical::unary::{UnaryCode, UnaryMode};
use egui::TextEdit;

use crate::ui_elements::{fill_code_columns, subheading};

use super::CodeFrame;

pub struct UnaryCodeFrame {
    code: UnaryCode,
}

impl Default for UnaryCodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for UnaryCodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.group(|ui| {
            ui.label(subheading("Mode"));
            ui.selectable_value(&mut self.code.mode, UnaryMode::Letter, "Letter");
            ui.selectable_value(&mut self.code.mode, UnaryMode::Word, "Word");
        });
        ui.add_space(16.0);

        match self.code.mode {
            UnaryMode::Letter => {
                ui.label("Alphabetical Mode: Provide an alphabet. Codes will be assigned to each character of the alphabet in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui
                    .add(TextEdit::singleline(&mut self.code.maps.alphabet))
                    .changed()
                {
                    self.code.set_letter_map();
                };
                fill_code_columns(16, 3, ui, Box::new(self.code.maps.chars_codes()));
            }
            UnaryMode::Word => {
                ui.label("Word Mode: Provide any number of words or phrases separated by commas. Codes will be assigned to each word or phrase in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui
                    .add(TextEdit::singleline(&mut self.code.maps.words_string))
                    .changed()
                {
                    self.code.set_word_map();
                };
                fill_code_columns(16, 3, ui, Box::new(self.code.maps.words_codes()));
            }
        }
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
