use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::{FibonacciCode, IOMode};
use eframe::egui::TextEdit;

impl ViewableCode for FibonacciCode {}

impl View for FibonacciCode {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.selectable_value(&mut self.mode, IOMode::Integer, "Integer");
        ui.selectable_value(&mut self.mode, IOMode::Letter, "Letter");
        ui.selectable_value(&mut self.mode, IOMode::Word, "Word");
        ui.add_space(16.0);

        match self.mode {
            IOMode::Letter => {
                ui.label("Alphabetical Mode: Provide an alphabet. Fibonacci codes will be assigned to each character of the alphabet in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui
                    .add(TextEdit::singleline(&mut self.maps.alphabet))
                    .changed()
                {
                    self.set_letter_map();
                };
                fill_code_columns(16, 5, ui, Box::new(self.maps.chars_codes()));
            }
            IOMode::Word => {
                ui.label("Word Mode: Provide any number of words or phrases separated by commas. Fibonacci codes will be assigned to each word or phrase in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui
                    .add(TextEdit::singleline(&mut self.maps.words_string))
                    .changed()
                {
                    self.set_word_map();
                };
                fill_code_columns(16, 5, ui, Box::new(self.maps.words_codes()));
            }
            IOMode::Integer => {
                ui.label("Integer Mode: Get the Fibonacci coding for any list of positive integers or decode any string of 0s and 1s into a list of positive integers. A sample list of encodings it provided below.");
                let pairs = (1..=64).map(|n| (n.to_string(), self.integer_code.encode_u32(n)));
                fill_code_columns(16, 5, ui, Box::new(pairs));
            }
        }
    }
}
