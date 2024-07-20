use egui::{DragValue, RichText};
use rand::{thread_rng, Rng};
use rngs::self_shrinking_generator::SelfShrinkingGenerator;
use utils::bits::Bit::{self, Zero};

use crate::ui_elements::{generate_random_u32s_box, UiElements};

use super::ClassicRngFrame;

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
            n_random: 5,
        }
    }
}

impl SelfShrinkingGeneratorFrame {}

impl ClassicRngFrame for SelfShrinkingGeneratorFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.subheading("Number of Bits");
        if ui
            .add(DragValue::new(&mut self.vector_length).clamp_range(4..=32))
            .changed()
        {
            self.rng.a.bits.truncate(self.vector_length);
            while self.rng.a.bits.len() < self.vector_length {
                self.rng.a.bits.push(Zero)
            }
            self.rng.a.taps.truncate(self.vector_length);
            while self.rng.a.taps.len() < self.vector_length {
                self.rng.a.taps.push(false)
            }
        };
        ui.add_space(16.0);

        ui.subheading("Internal State");
        ui.label("Bits of state along the top row with the tagged bits marked on the second row. New bits are pushed in from the left.");
        ui.add_space(8.0);
        if ui.button("next bit").clicked() {
            self.rng.next_bit();
        }
        if ui.button("step").clicked() {
            self.rng.step();
        }
        ui.add_space(8.0);
        egui::Grid::new("ssg_a_grid")
            .num_columns(self.vector_length)
            .max_col_width(5.0)
            .min_col_width(5.0)
            .show(ui, |ui| {
                for b in self.rng.a.bits.iter_mut() {
                    let x = RichText::from(b.to_string()).monospace().size(12.0);
                    if ui.button(x).clicked() {
                        b.flip()
                    }
                }
                ui.end_row();
                for t in self.rng.a.taps.iter_mut() {
                    match t {
                        true => {
                            if ui
                                .button(RichText::from("^").monospace().size(12.0))
                                .clicked()
                            {
                                *t = false
                            }
                        }
                        false => {
                            if ui
                                .button(RichText::from("_").monospace().size(12.0))
                                .clicked()
                            {
                                *t = true
                            }
                        }
                    }
                }
            });

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
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
