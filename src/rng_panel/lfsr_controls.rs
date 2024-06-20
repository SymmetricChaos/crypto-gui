use egui::{DragValue, RichText};
use rand::{thread_rng, Rng};
use rngs::{lfsr::Lfsr, ClassicRng};
use utils::bits::Bit;

use crate::ui_elements::{generate_random_u32s_box, UiElements};

use super::ClassicRngFrame;

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
        ui.subheading("Number of Bits");
        if ui
            .add(DragValue::new(&mut self.vector_length).clamp_range(4..=32))
            .changed()
        {
            self.rng.bits.truncate(self.vector_length);
            while self.rng.bits.len() < self.vector_length {
                self.rng.bits.push(utils::bits::Bit::Zero)
            }
            self.rng.taps.truncate(self.vector_length);
            while self.rng.taps.len() < self.vector_length {
                self.rng.taps.push(false)
            }
        };
        ui.add_space(16.0);

        ui.subheading("Internal State");
        ui.label("Bits of state along the top row with the tagged bits marked on the second row. New bits are pushed in from the left.");
        ui.add_space(8.0);
        if ui.button("step").clicked() {
            self.rng.next_u32();
        }
        ui.add_space(8.0);
        egui::Grid::new("lfsr_grid")
            .num_columns(self.vector_length)
            .max_col_width(5.0)
            .min_col_width(5.0)
            .show(ui, |ui| {
                for b in self.rng.bits.iter_mut() {
                    let x = RichText::from(b.to_string()).monospace().size(12.0);
                    if ui.button(x).clicked() {
                        b.flip()
                    }
                }
                ui.end_row();
                for t in self.rng.taps.iter_mut() {
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
        ui.subheading(format!("Next Bit: {}", self.rng.peek_next_bit()));

        // ui.add_space(16.0);
        // ui.subheading("Current State as an Integer");
        // match self.rng.big_endian {
        //     true => {
        //         ui.label(format!("{}", bits_to_u32_be(&self.rng.bits)));
        //     }
        //     false => {
        //         ui.label(format!("{}", bits_to_u32_le(&self.rng.bits)));
        //     }
        // }
        ui.add_space(16.0);
        generate_random_u32s_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
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
