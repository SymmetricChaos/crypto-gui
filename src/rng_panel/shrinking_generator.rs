use egui::{DragValue, RichText};
use rand::{thread_rng, Rng};
use rngs::shrinking_generator::ShrinkingGenerator;
use utils::bits::Bit::{self, Zero};

use crate::ui_elements::{generate_random_u32s_box, UiElements};

use super::ClassicRngFrame;

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
        ui.subheading("Number of Bits (A)");
        if ui
            .add(DragValue::new(&mut self.vector_length_a).clamp_range(4..=32))
            .changed()
        {
            self.rng.a.bits.truncate(self.vector_length_a);
            while self.rng.a.bits.len() < self.vector_length_a {
                self.rng.a.bits.push(Zero)
            }
            self.rng.a.taps.truncate(self.vector_length_a);
            while self.rng.a.taps.len() < self.vector_length_a {
                self.rng.a.taps.push(false)
            }
        };
        ui.add_space(8.0);
        ui.subheading("Number of Bits (S)");
        if ui
            .add(DragValue::new(&mut self.vector_length_s).clamp_range(4..=32))
            .changed()
        {
            self.rng.s.bits.truncate(self.vector_length_s);
            while self.rng.s.bits.len() < self.vector_length_s {
                self.rng.s.bits.push(Zero)
            }
            self.rng.s.taps.truncate(self.vector_length_s);
            while self.rng.s.taps.len() < self.vector_length_s {
                self.rng.s.taps.push(false)
            }
        };
        ui.add_space(16.0);

        ui.subheading("Internal State");
        ui.label("Bits of state along the top row with the tagged bits marked on the second row. New bits are pushed in from the left.");
        ui.add_space(8.0);
        ui.subheading("Generator A");
        egui::Grid::new("shrg_a_grid")
            .num_columns(self.vector_length_a)
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

        ui.add_space(8.0);
        ui.subheading("Generator S");
        egui::Grid::new("shrg_s_grid")
            .num_columns(self.vector_length_a)
            .max_col_width(5.0)
            .min_col_width(5.0)
            .show(ui, |ui| {
                for b in self.rng.s.bits.iter_mut() {
                    let x = RichText::from(b.to_string()).monospace().size(12.0);
                    if ui.button(x).clicked() {
                        b.flip()
                    }
                }
                ui.end_row();
                for t in self.rng.s.taps.iter_mut() {
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
