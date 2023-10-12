use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::{letter_word_code::IOMode, mathematical::roman_numeral::RomanNumeral};
use egui::TextEdit;
use utils::text_functions::unique_string;

pub struct RomanNumeralFrame {
    code: RomanNumeral,
    words_string: String,
}

impl Default for RomanNumeralFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            words_string: String::new(),
        }
    }
}

impl CodeFrame for RomanNumeralFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.add_space(16.0);
        ui.group(|ui| {
            ui.subheading("Mode");
            ui.selectable_value(&mut self.code.mode, IOMode::Integer, "Integer");
            ui.selectable_value(&mut self.code.mode, IOMode::Letter, "Letter");
            ui.selectable_value(&mut self.code.mode, IOMode::Word, "Word");
        });
        ui.add_space(16.0);

        match self.code.mode {
            IOMode::Letter => {
                ui.label("Provide an alphabet. Roman numerals, starting with one, will be assigned to each character.");
                if ui.control_string(&mut self.code.maps.alphabet).changed() {
                    unique_string(&mut self.code.maps.alphabet);
                    self.code.maps.alphabet.retain(|x| x != '�');
                };
                // ui.fill_code_columns(16, 5, Box::new(self.code.maps.chars_codes()));
            }
            IOMode::Word => {
                ui.label("Provide any number of words or phrases separated by commas. Roman numerals, starting with one, will be assigned to each word.");
                if ui
                    .add(TextEdit::multiline(&mut self.words_string))
                    .changed()
                {
                    self.code.maps.set_words(&self.words_string);
                };
                // ui.fill_code_columns(16, 5, Box::new(self.code.maps.words_codes()));
            }
            IOMode::Integer => {
                ui.label("Convert between standard numbers and their representation as Roman Numerals. The first 16 encodings appear below.");
                let pairs =
                    (1..17).map(|n| (n.to_string(), RomanNumeral::encode_int(n as usize).unwrap()));
                ui.fill_code_columns(16, 5, Box::new(pairs));
            }
        }
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
