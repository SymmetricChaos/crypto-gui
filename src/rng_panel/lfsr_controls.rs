use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, lfsr_grid_controls};
use rand::{thread_rng, Rng};
use rngs::lfsr::Lfsr;
use utils::bits::Bit::{self};

pub struct LfsrFrame {
    rng: Lfsr,
    vector_length: usize,
    randoms: String,
    n_random: usize,
}

impl Default for LfsrFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            vector_length: 16,
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl LfsrFrame {}

impl ClassicRngFrame for LfsrFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/lfsr.rs",
        );
        ui.add_space(8.0);

        match self.rng.mode {
            rngs::lfsr::LfsrMode::Fibonncci => ui.label("In Fibonacci mode the LFSR outputs the XOR of each tapped bit then shifts the register left, inserting the output bit on the right."),
            rngs::lfsr::LfsrMode::Galois => ui.label("In Galois mode the LFSR outputs the leftmost bit, XORs it into each tap, then shifts the register left, inserting the output bit on the right."),
        };

        lfsr_grid_controls(ui, &mut self.rng, &mut self.vector_length, "lfsr_grid");

        ui.add_space(16.0);
        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        &mut self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        for b in self.rng.bits.iter_mut() {
            *b = Bit::from(rng.gen_bool(0.5));
        }
        for t in self.rng.taps.iter_mut() {
            *t = rng.gen_bool(0.15);
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
