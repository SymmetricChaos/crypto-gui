use codes::ecc::parity_bit::ParityBit;
use egui::Slider;
use utils::bits::Bit;

use crate::ui_elements::UiElements;

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
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/ecc/parity_bit.rs",
        );
        ui.add_space(8.0);

        ui.subheading("Data Bits");
        ui.add(Slider::new(&mut self.code.block_size, 1..=10));

        ui.add_space(8.0);

        ui.subheading("Parity Bit Position");
        ui.add(Slider::new(
            &mut self.code.position,
            0..=self.code.block_size,
        ));

        ui.add_space(8.0);

        ui.subheading("Parity");
        ui.label("Even parity means the extra bit ensures there are an even number of 1s. Odd parity means there will be an odd number of 1s.");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.code.parity, Bit::Zero, "Even");
            ui.selectable_value(&mut self.code.parity, Bit::One, "Odd");
        });

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
