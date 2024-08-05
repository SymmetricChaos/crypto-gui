use super::CodeFrame;
use crate::ui_elements::{integer_letter_code_controls, integer_word_code_controls, UiElements};
use codes::{letter_word_code::IOMode, mathematical::symmetric_unary::SymmetricUnaryCode};
use strum::IntoEnumIterator;

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
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/symmetric_unary.rs",
        );
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.subheading("Mode");
            for variant in IOMode::iter() {
                ui.selectable_value(&mut self.code.mode, variant, variant.to_string());
            }
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
                integer_letter_code_controls(ui, &mut self.code.maps.alphabet);
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
                integer_word_code_controls(ui, &mut self.words_string, &mut self.code.maps);
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
                ui.label("Convert between numbers and their symmetric unary encodings.");
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
