use super::CodeFrame;
use crate::ui_elements::{integer_letter_code_controls, integer_word_code_controls, UiElements};
use codes::{letter_word_code::IOMode, mathematical::roman_numeral::RomanNumeral};
use strum::IntoEnumIterator;

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
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/roman_numeral.rs",
        );
        ui.add_space(8.0);

        ui.add_space(16.0);
        ui.group(|ui| {
            ui.subheading("Mode");
            for variant in IOMode::iter() {
                ui.selectable_value(&mut self.code.mode, variant, variant.to_string());
            }
        });
        ui.add_space(16.0);

        match self.code.mode {
            IOMode::Letter => {
                integer_letter_code_controls(ui, &mut self.code.maps.alphabet);
                // ui.fill_code_columns(16, 5, Box::new(self.code.maps.chars_codes()));
            }
            IOMode::Word => {
                integer_word_code_controls(ui, &mut self.words_string, &mut self.code.maps);
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
