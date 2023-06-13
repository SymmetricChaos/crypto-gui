use codes::ecc::parity_check::ParityBit;
use egui::Slider;

use super::CodeFrame;

pub struct ParityBitFrame {
    code: ParityBit,
}

impl Default for ParityBitFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for ParityBitFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.label("Data Bits");
        ui.add(Slider::new(&mut self.code.block_size, 1..=10));

        ui.add_space(8.0);

        ui.label("Parity Bit Position");
        ui.add(Slider::new(
            &mut self.code.position,
            0..=self.code.block_size,
        ));

        ui.add_space(8.0);

        ui.checkbox(&mut self.code.inverted, "Odd Parity Bit");
        ui.label("An odd parity bit ensures that there are an odd number of bits set rather than an even number.");
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
