use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::FibonacciCode;
use eframe::egui::TextEdit;

impl ViewableCode for FibonacciCode {}

impl View for FibonacciCode {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        if ui.button("Switch Mode").clicked() {
            self.integer_mode = !self.integer_mode;
        }
        ui.add_space(16.0);

        if self.integer_mode {
            ui.label("Integer Mode: get the Fibonacci coding for any list of positive integers or decode any string of 0s and 1s into a list of positive integers. A sample list of encodings it provided below.");
            let pairs = (0..64).map(|n| (n.to_string(), self.integer_code.encode_u32(n)));
            fill_code_columns(16, 5, ui, Box::new(pairs));
        } else {
            ui.label("Alphabetical Mode: Provide an alphabet. Fibonacci codes will be assigned to each character of the alphabet in ascending order. The alphabet and the code for each letter is provided below.");
            if ui.add(TextEdit::singleline(&mut self.alphabet)).changed() {
                self.set_map();
            };
            fill_code_columns(16, 5, ui, Box::new(self.chars_codes()));
        }
    }
}
