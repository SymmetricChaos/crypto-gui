use super::ClassicRngFrame;
use crate::ui_elements::{lfsr_grid_controls, UiElements};
use egui::DragValue;
use rand::{thread_rng, Rng};
use rngs::{shrinking_generator::ShrinkingGenerator, ClassicRng};
use utils::bits::Bit::{self};

pub struct ShrinkingGeneratorFrame {
    rng: ShrinkingGenerator,
    vector_length_a: usize,
    vector_length_s: usize,
    randoms: String,
    n_random: usize,
    err: bool,
}

impl Default for ShrinkingGeneratorFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            vector_length_a: 32,
            vector_length_s: 29,
            randoms: String::new(),
            n_random: 5,
            err: false,
        }
    }
}

impl ShrinkingGeneratorFrame {}

impl ClassicRngFrame for ShrinkingGeneratorFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/shrinking_generator.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_rng(self);
        ui.add_space(8.0);

        ui.subheading("Generator A");
        lfsr_grid_controls(
            ui,
            &mut self.rng.a,
            &mut self.vector_length_a,
            "lfsr_grid_a",
        );

        ui.add_space(32.0);
        ui.subheading("Generator S");
        lfsr_grid_controls(
            ui,
            &mut self.rng.s,
            &mut self.vector_length_s,
            "lfsr_grid_s",
        );

        ui.add_space(32.0);

        if ui.button("step together").clicked() {
            self.rng.step();
        }

        ui.add_space(32.0);

        let n_random: &mut usize = &mut self.n_random;
        let randoms: &mut String = &mut self.randoms;
        ui.horizontal(|ui| {
            if ui.button("Random Numbers").clicked() {
                self.err = false;
                if !self.rng.a.bits.contains(&Bit::One)
                    || !self.rng.a.taps.contains(&true)
                    || !self.rng.s.bits.contains(&Bit::One)
                    || !self.rng.s.taps.contains(&true)
                {
                    self.err = true;
                }
                if !self.err {
                    for _ in 0..*n_random {
                        if !randoms.is_empty() {
                            randoms.push_str(", ");
                        }
                        randoms.push_str(&self.rng.next_u32().to_string());
                    }
                }
            }
            ui.add(DragValue::new(n_random).range(1..=100))
        });

        ui.text_edit_multiline(randoms);
        ui.add_space(8.0);
        if self.err {
            ui.error_text(
                "Both generators must have at least one tap selected and at least one bit set.",
            );
        }
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
