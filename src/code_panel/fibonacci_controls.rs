use super::CodeFrame;
use crate::ui_elements::{integer_letter_code_controls, integer_word_code_controls, UiElements};
use codes::{letter_word_code::IOMode, mathematical::fibonacci::FibonacciCode};
use strum::IntoEnumIterator;

pub struct FibonacciCodeFrame {
    code: FibonacciCode,
    words_string: String,
}

impl Default for FibonacciCodeFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            words_string: String::new(),
        }
    }
}

impl CodeFrame for FibonacciCodeFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/fibonacci.rs",
        );
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.subheading("Mode");
            for variant in IOMode::iter() {
                ui.selectable_value(&mut self.code.mode, variant, variant.to_string());
            }
        });
        ui.add_space(8.0);
        ui.checkbox(&mut self.code.spaced, "Use Spaces");
        ui.add_space(8.0);

        match self.code.mode {
            IOMode::Letter => {
                integer_letter_code_controls(ui, &mut self.code.maps.alphabet);

                let pairs = self.code.maps.alphabet.chars().enumerate().map(|(a, b)| {
                    (
                        b,
                        self.code
                            .integer_code
                            .borrow_mut()
                            .encode_u32((a as u32) + 1)
                            .to_owned(),
                    )
                });
                ui.two_column_table("Character", "Code", Box::new(pairs));
            }
            IOMode::Word => {
                integer_word_code_controls(ui, &mut self.words_string, &mut self.code.maps);
                let pairs = self.code.maps.words.iter().enumerate().map(|(a, b)| {
                    (
                        b,
                        self.code
                            .integer_code
                            .borrow_mut()
                            .encode_u32((a as u32) + 1)
                            .to_owned(),
                    )
                });
                ui.two_column_table("Word", "Code", Box::new(pairs));
            }
            IOMode::Integer => {
                ui.label("Get the Fibonacci coding for any list of positive integers or decode any string of 0s and 1s into a list of positive integers. A sample list of encodings is provided below.");
                let pairs = (1..=64).map(|n| {
                    (
                        n.to_string(),
                        self.code.integer_code.borrow_mut().encode_u32(n).to_owned(),
                    )
                });
                ui.add_space(16.0);
                ui.two_column_table("Integer", "Code", Box::new(pairs));
            }
        }
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
