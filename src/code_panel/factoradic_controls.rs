use super::CodeFrame;
use crate::ui_elements::{integer_letter_code_controls, integer_word_code_controls, UiElements};
use codes::{letter_word_code::IOMode, mathematical::factoradic::Factoradic};
use strum::IntoEnumIterator;

pub struct FactoradicFrame {
    code: Factoradic,
    words_string: String,
}

impl Default for FactoradicFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            words_string: String::new(),
        }
    }
}

impl CodeFrame for FactoradicFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/factoradic.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Example");
        ui.mono(
            "factoradic number     3:4:2:2:1:0\n\ndigits                3   4  2  2  1  0\nplace values       ×120 ×24 ×6 ×2 ×1 ×1\ndigit values        360  96 12  4  1  0\n\ntheir sum is 473",
        );

        ui.group(|ui| {
            ui.subheading("Mode");
            for variant in IOMode::iter() {
                ui.selectable_value(&mut self.code.mode, variant, variant.to_string());
            }
        });
        ui.add_space(8.0);

        match self.code.mode {
            IOMode::Letter => {
                integer_letter_code_controls(ui, &mut self.code.maps.alphabet);
                ui.two_column_table(
                    "Character",
                    "Code",
                    Box::new(
                        self.code
                            .maps
                            .ints_chars()
                            .map(|(a, b)| (b, self.code.encode_usize(a))),
                    ),
                );
            }
            IOMode::Word => {
                integer_word_code_controls(ui, &mut self.words_string, &mut self.code.maps);

                ui.two_column_table(
                    "Word",
                    "Code",
                    Box::new(
                        self.code
                            .maps
                            .ints_words()
                            .map(|(a, b)| (b, self.code.encode_usize(a))),
                    ),
                );
            }
            IOMode::Integer => {
                ui.label("Convert between numbers and their factoradic encodings. When decoding the '�' symbol appears when an invalid code is encoutered.");
                ui.add_space(16.0);
                ui.two_column_table(
                    "Integer",
                    "Code",
                    Box::new((0..10).into_iter().map(|n| (n, self.code.encode_usize(n)))),
                );
            }
        }
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
