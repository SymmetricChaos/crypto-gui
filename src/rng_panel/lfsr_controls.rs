use egui::{DragValue, RichText};
use rand::{thread_rng, Rng};
use rngs::{lfsr::Lfsr, ClassicRng};
use utils::bits::{bits_to_int_big_endian, bits_to_int_little_endian, Bit};

use crate::ui_elements::UiElements;

use super::ClassicRngFrame;

pub struct LfsrFrame {
    rng: Lfsr,
    vector_length: usize,
}

impl Default for LfsrFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            vector_length: 16,
        }
    }
}

impl LfsrFrame {}

impl ClassicRngFrame for LfsrFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.subheading("Number of Bits");
        if ui
            .add(DragValue::new(&mut self.vector_length).clamp_range(8..=20))
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
        ui.label("Bits of state along the top row. And the bits tapped along the bottom row.");
        ui.add_space(8.0);
        if ui.button("step").clicked() {
            self.rng.step();
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

        ui.subheading(format!("Next Bit: {}", self.rng.next_bit()));

        ui.add_space(16.0);
        ui.subheading("Current State as an Integer");
        ui.label(format!(
            "{} (big endian)",
            bits_to_int_big_endian(&self.rng.bits)
        ));
        ui.label(format!(
            "{} (little endian)",
            bits_to_int_little_endian(&self.rng.bits)
        ));
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
