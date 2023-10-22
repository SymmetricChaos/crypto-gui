use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::{letter_word_code::IOMode, mathematical::symmetric_unary::SymmetricUnaryCode};
use egui::TextEdit;
use utils::text_functions::unique_string;

pub struct SymUnaryCodeFrame {
    code: SymmetricUnaryCode,
    words_string: String,
}

impl Default for SymUnaryCodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            words_string: String::new(),
        }
    }
}

impl SymUnaryCodeFrame {
    pub fn usize_to_unary(&self, n: usize) -> String {
        if self.code.invert {
            if n == 0 {
                return String::from("0");
            } else {
                format!("1{}1", "0".repeat(n - 1))
            }
        } else {
            if n == 0 {
                return String::from("1");
            } else {
                format!("0{}0", "1".repeat(n - 1))
            }
        }
    }
}

impl CodeFrame for SymUnaryCodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.subheading("Mode");
            ui.selectable_value(&mut self.code.mode, IOMode::Integer, "Integer");
            ui.selectable_value(&mut self.code.mode, IOMode::Letter, "Letter");
            ui.selectable_value(&mut self.code.mode, IOMode::Word, "Word");
        });

        // ui.add_space(8.0);
        // ui.checkbox(&mut self.code.spaced, "Use Spaces");
        // ui.add_space(8.0);

        ui.subheading("Invert Bits");
        ui.label("The 0 and 1 bits can be switched to create an equivalent code.");
        ui.checkbox(&mut self.code.invert, "");
        ui.add_space(8.0);

        match self.code.mode {
            IOMode::Letter => {
                ui.label("Provide an alphabet. Codes will be assigned to each character of the alphabet in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui.control_string(&mut self.code.maps.alphabet).changed() {
                    unique_string(&mut self.code.maps.alphabet);
                    self.code.maps.alphabet.retain(|x| x != '�');
                };
                ui.add_space(16.0);
                ui.two_column_table(
                    "Code",
                    "Character",
                    Box::new(
                        self.code
                            .maps
                            .ints_chars()
                            .map(|(a, b)| (self.usize_to_unary(a), b)),
                    ),
                );
            }
            IOMode::Word => {
                ui.label("Provide any number of words or phrases separated by commas. Codes will be assigned to each word or phrase in ascending order. When decoding the '�' symbol appears when a code without a known meaning is assigned.");
                if ui
                    .add(TextEdit::singleline(&mut self.words_string))
                    .changed()
                {
                    self.code.maps.set_words(&self.words_string);
                };
                ui.add_space(16.0);
                ui.two_column_table(
                    "Code",
                    "Word",
                    Box::new(
                        self.code
                            .maps
                            .ints_words()
                            .map(|(a, b)| (self.usize_to_unary(a), b)),
                    ),
                );
            }
            IOMode::Integer => {
                ui.label("Convert between numbers and their symmetric unary encodings. When decoding the '�' symbol appears when an invalid code is encoutered.");
                ui.add_space(16.0);
                ui.two_column_table(
                    "Code",
                    "Integer",
                    Box::new((0..6).into_iter().map(|n| (n, self.usize_to_unary(n)))),
                );
            }
        }
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
