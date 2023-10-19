use codes::mathematical::twos_complement::TwosComplement;

use super::CodeFrame;
use crate::ui_elements::UiElements;

pub struct TwosComplementFrame {
    code: TwosComplement,
}

impl Default for TwosComplementFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for TwosComplementFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.add_space(16.0);

        ui.subheading("Example (8=bits)");
        ui.mono(
            "number             56
        binary       00111000
        bits flipped 11000111
        plus one     11001000",
        );

        ui.label("Convert between \"standard\" base-10 numbers and their representation as two's complement. Encoding is done uses 32-bits but could be done with any number of bits.");
        let pairs = (-15..=16).map(|n| (n.to_string(), TwosComplement::encode_i32(n)));
        ui.fill_code_columns(16, 5, Box::new(pairs));

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
