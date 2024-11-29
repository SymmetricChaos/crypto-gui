use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::mathematical::roman_numeral::RomanNumeral;

pub struct RomanNumeralFrame {
    code: RomanNumeral,
}

impl Default for RomanNumeralFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
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

        ui.label("Convert between standard numbers and their representation as Roman Numerals. The first 16 encodings appear below.");
        let pairs = (1..17).map(|n| (n.to_string(), RomanNumeral::encode_int(n as usize).unwrap()));
        ui.fill_code_columns(16, 5, Box::new(pairs));

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
