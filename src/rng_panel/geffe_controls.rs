use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, lfsr_grid_controls, UiElements};
use rand::{thread_rng, Rng};
use rngs::geffe::Geffe;
use utils::bits::Bit;

pub struct GeffeFrame {
    rng: Geffe,
    vector_lengths: [usize; 3],
    randoms: String,
    n_random: usize,
}

impl Default for GeffeFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            vector_lengths: [16, 16, 16],
            randoms: String::new(),
            n_random: 1,
        }
    }
}

impl GeffeFrame {}

impl ClassicRngFrame for GeffeFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/geffe.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_rng(self);
        ui.add_space(8.0);

        for i in 0..3 {
            let lfsr = &mut self.rng.rngs[i];

            lfsr_grid_controls(
                ui,
                lfsr,
                &mut self.vector_lengths[i],
                &format!("lfsr_grid{}", i),
            );

            ui.add_space(8.0);
        }

        ui.subheading("Multiplexer State");
        let (a, b, c) = (
            self.rng.rngs[0].peek_next_bit(),
            self.rng.rngs[1].peek_next_bit(),
            self.rng.rngs[2].peek_next_bit(),
        );
        ui.label(format!("({a} & {b}) ⊕ (¬{a} & {c})"));
        ui.label(format!("Next Bit: {}", self.rng.peek_next_bit()));

        ui.add_space(8.0);
        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&mut self) -> &mut dyn rngs::SimpleRng {
        &mut self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        for lfsr in self.rng.rngs.iter_mut() {
            for b in lfsr.bits.iter_mut() {
                *b = Bit::from(rng.gen_bool(0.5));
            }
            for t in lfsr.taps.iter_mut() {
                *t = rng.gen_bool(0.15);
            }
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
