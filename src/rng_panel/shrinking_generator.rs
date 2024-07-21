use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, lfsr_grid_controls, UiElements};
use rand::{thread_rng, Rng};
use rngs::shrinking_generator::ShrinkingGenerator;
use utils::bits::Bit::{self};

pub struct ShrinkingGeneratorFrame {
    rng: ShrinkingGenerator,
    vector_length_a: usize,
    vector_length_s: usize,
    randoms: String,
    n_random: usize,
}

impl Default for ShrinkingGeneratorFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            vector_length_a: 16,
            vector_length_s: 16,
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl ShrinkingGeneratorFrame {}

impl ClassicRngFrame for ShrinkingGeneratorFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.subheading("Generator A");
        lfsr_grid_controls(
            ui,
            &mut self.rng.a,
            &mut self.vector_length_a,
            "lfsr_grid_a",
        );

        ui.add_space(8.0);
        ui.subheading("Generator S");
        lfsr_grid_controls(
            ui,
            &mut self.rng.s,
            &mut self.vector_length_s,
            "lfsr_grid_s",
        );

        ui.add_space(8.0);
        if ui.button("next bit").clicked() {
            self.rng.next_bit();
        }
        if ui.button("step").clicked() {
            self.rng.step();
        }

        ui.add_space(16.0);
        generate_random_u32s_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        for b in self.rng.a.bits.iter_mut() {
            *b = Bit::from(rng.gen_bool(0.5));
        }
        for t in self.rng.a.taps.iter_mut() {
            *t = rng.gen_bool(0.15);
        }
        for b in self.rng.s.bits.iter_mut() {
            *b = Bit::from(rng.gen_bool(0.5));
        }
        for t in self.rng.s.taps.iter_mut() {
            *t = rng.gen_bool(0.15);
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
