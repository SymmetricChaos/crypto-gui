use codes::ecc::hamming::HammingCode;

use super::CodeFrame;

pub struct HammingFrame {
    code: HammingCode,
}

impl Default for HammingFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for HammingFrame {
    fn ui(&mut self, _ui: &mut egui::Ui, _errors: &mut String) {
        // ui.label("Check Bits");
        // ui.add(Slider::new(&mut self.code.check_bits, 3..=5));
        // ui.add_space(8.0);
        // ui.checkbox(&mut self.code.parity_bit, "Extra Parity Bit");
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
