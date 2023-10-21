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

        ui.subheading("Example of Finding the Negative (8-bits)");
        ui.label("The additive inverse (negative) of a number in two's complement is found by inverting all of the bits and then adding one to the result.");
        ui.mono("number       00111000\nbits flipped 11000111\nplus one     11001000");
        ui.label("Adding these numbers in the usual way, but ignoring any carry past the last bit, gives all zeroes.");
        ui.mono(" 00111000\n+11001000\n 00000000");
        ui.label("So if we say that 00111000 = 56 it must be that 11001000 = -56.");
        ui.add_space(16.0);

        ui.subheading("Exception: Most Negative Number (8-bits)");
        ui.label("The most negative number in two's complement is unlike others because its additive inverse is itself a property that only zero has in standard arithmetic.");
        ui.mono("number       10000000\nbits flipped 01111111\nplus one     10000000");
        ui.label("because of this programs usually assume either that the most negaative number will never be inverted or crash if it is.");
        ui.add_space(16.0);

        ui.label("Convert between \"standard\" base-10 numbers and their representation as two's complement. Encoding is done uses 32-bits but could be done with any number of bits.");
        let pairs = (-15..=16).map(|n| (n.to_string(), TwosComplement::encode_i32(n)));
        ui.fill_code_columns(16, 5, Box::new(pairs));

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
