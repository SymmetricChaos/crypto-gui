use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, lfsr_grid_controls, UiElements};
use rand::{thread_rng, Rng};
use rngs::self_shrinking_generator::SelfShrinkingGenerator;
use utils::bits::Bit::{self};

pub struct SelfShrinkingGeneratorFrame {
    rng: SelfShrinkingGenerator,
    vector_length: usize,
    randoms: String,
    n_random: usize,
}

impl Default for SelfShrinkingGeneratorFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            vector_length: 16,
            randoms: String::new(),
            n_random: 1,
        }
    }
}

impl SelfShrinkingGeneratorFrame {}

impl ClassicRngFrame for SelfShrinkingGeneratorFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/self_shrinking_generator.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_rng(self);
        ui.add_space(8.0);

        if ui.button("step").clicked() {
            self.rng.step();
        }
        ui.add_space(8.0);
        lfsr_grid_controls(ui, &mut self.rng.a, &mut self.vector_length, "lfsr_grid");

        ui.add_space(16.0);
        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        &mut self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        for b in self.rng.a.bits.iter_mut() {
            *b = Bit::from(rng.gen_bool(0.5));
        }
        for t in self.rng.a.taps.iter_mut() {
            *t = rng.gen_bool(0.15);
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
