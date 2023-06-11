use codes::ecc::hamming::{HammingCode, GEN_4_7, GEN_4_8};

use crate::egui_aux::mono;

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
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.checkbox(&mut self.code.extra_bit, "Extra Parity Bit");
        ui.add_space(16.0);

        ui.label("Generator Matrix");
        match self.code.extra_bit {
            true => ui.label(mono(GEN_4_8.to_string())),
            false => ui.label(mono(GEN_4_7.to_string())),
        };
        ui.add_space(16.0);
        ui.label("The first four columns are the identity matrix. The last three columns show which of the data bits are covered by each parity bit. For instance the fifth column (controlling the first parity bit) is [1 1 0 1] because the first parity bit covers the first, second, and fourth data bits.");
        ui.label("Columns can be rearranged to produce codes with equivalent error correcting properties.");
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
