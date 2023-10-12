use crate::ui_elements::UiElements;

use super::CodeFrame;
use codes::{letter_word_code::IOMode, mathematical::levenshtein::LevenshteinCode, traits::Code};
use egui::TextEdit;
use utils::text_functions::unique_string;

pub struct LevenshteinCodeFrame {
    code: LevenshteinCode,
    words_string: String,
}

impl Default for LevenshteinCodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            words_string: String::new(),
        }
    }
}

impl CodeFrame for LevenshteinCodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.subheading("Mode");
            ui.selectable_value(&mut self.code.mode, IOMode::Integer, "Integer");
            ui.selectable_value(&mut self.code.mode, IOMode::Letter, "Letter");
            ui.selectable_value(&mut self.code.mode, IOMode::Word, "Word");
        });
        ui.add_space(16.0);

        match self.code.mode {
            IOMode::Letter => {
                ui.label("Provide an alphabet. Levenshtein codes will be assigned to each character of the alphabet in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui.control_string(&mut self.code.maps.alphabet).changed() {
                    unique_string(&mut self.code.maps.alphabet);
                    self.code.maps.alphabet.retain(|x| x != '�');
                };
                ui.add_space(16.0);
                // ui.two_column_table("Code", "Character", Box::new(self.code.maps.chars_codes()));
            }
            IOMode::Word => {
                ui.label("Provide any number of words or phrases separated by commas. Levenshtein codes will be assigned to each word or phrase in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui
                    .add(TextEdit::multiline(&mut self.words_string))
                    .changed()
                {
                    self.code.maps.set_words(&self.words_string);
                };
                ui.add_space(16.0);
                // ui.two_column_table("Code", "Word", Box::new(self.code.maps.words_codes()));
            }
            IOMode::Integer => {
                ui.label("Get the Levenshtein coding for any list of non-negative integers or decode any string of 0s and 1s into a list of non-negative integers. A sample list of encodings it provided below.");
                let pairs = (0..32).map(|n| (n.to_string(), self.code.integer_code.encode_u32(n)));
                ui.add_space(16.0);
                ui.two_column_table("Code", "Integer", Box::new(pairs));
            }
        }
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn Code {
        &self.code
    }
}
