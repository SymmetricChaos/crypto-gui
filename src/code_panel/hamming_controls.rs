use codes::ecc::hamming::{HammingCode, GEN_4_7_MIX, GEN_4_7_SYS, GEN_4_8_MIX, GEN_4_8_SYS};

use crate::ui_elements::mono;

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
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.code.extra_bit, "Extra Parity Bit");
        ui.label("When this is checked one additional parity bit is included. That allows detecting, but not correcting, two bit errors.");
        ui.add_space(16.0);

        ui.checkbox(&mut self.code.systematic, "Systematic Encoding");
        ui.label("When this is checked the code is organized so that the data bits and parity bits are separated. When unchecked data and parity bits are mixed so that the error syndrome is the index of the error, written in binary.");
        ui.add_space(16.0);

        ui.label("Generator Matrix");
        ui.label(mono(match self.code.extra_bit {
            true => match self.code.systematic {
                true => GEN_4_8_SYS.to_string(),
                false => GEN_4_8_MIX.to_string(),
            },
            false => match self.code.systematic {
                true => GEN_4_7_SYS.to_string(),
                false => GEN_4_7_MIX.to_string(),
            },
        }));
        ui.add_space(16.0);
        ui.label("The columns with one bit set capture the data bits. The columns with multiple bits set check the parity of the selected data bits.");
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
