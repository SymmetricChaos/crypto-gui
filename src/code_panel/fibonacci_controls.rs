use crate::ui_elements::UiElements;

use super::CodeFrame;
use codes::{mathematical::fibonacci::FibonacciCode, traits::IOMode};
use egui::TextEdit;

pub struct FibonacciCodeFrame {
    code: FibonacciCode,
}

impl Default for FibonacciCodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for FibonacciCodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.group(|ui| {
            ui.subheading("Mode");
            ui.selectable_value(&mut self.code.mode, IOMode::Integer, "Integer");
            ui.selectable_value(&mut self.code.mode, IOMode::Letter, "Letter");
            ui.selectable_value(&mut self.code.mode, IOMode::Word, "Word");
        });
        ui.add_space(16.0);

        match self.code.mode {
            IOMode::Letter => {
                ui.label("Alphabetical Mode: Provide an alphabet. Fibonacci codes will be assigned to each character of the alphabet in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui
                    .add(TextEdit::singleline(&mut self.code.maps.alphabet))
                    .changed()
                {
                    self.code.set_letter_map();
                };
                ui.fill_code_columns(16, 5, Box::new(self.code.maps.chars_codes()));
            }
            IOMode::Word => {
                ui.label("Word Mode: Provide any number of words or phrases separated by commas. Fibonacci codes will be assigned to each word or phrase in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui
                    .add(TextEdit::singleline(&mut self.code.maps.words_string))
                    .changed()
                {
                    self.code.set_word_map();
                };
                ui.fill_code_columns(16, 5, Box::new(self.code.maps.words_codes()));
            }
            IOMode::Integer => {
                ui.label("Integer Mode: Get the Fibonacci coding for any list of positive integers or decode any string of 0s and 1s into a list of positive integers. A sample list of encodings it provided below.");
                let pairs = (1..=64).map(|n| (n.to_string(), self.code.integer_code.encode_u32(n)));
                ui.fill_code_columns(16, 5, Box::new(pairs));
            }
        }
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
