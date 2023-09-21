use egui::DragValue;
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
        if ui
            .add(DragValue::new(&mut self.vector_length).clamp_range(8..=16))
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

        ui.horizontal(|ui| {
            for b in self.rng.bits.iter_mut() {
                if ui.small_button(b.to_string()).clicked() {
                    b.flip()
                }
            }
        });
        ui.horizontal(|ui| {
            for t in self.rng.taps.iter_mut() {
                match t {
                    true => {
                        if ui.small_button("↑").clicked() {
                            *t = false
                        }
                    }
                    false => {
                        if ui.small_button("⋅").clicked() {
                            *t = true
                        }
                    }
                }
            }
        });

        ui.subheading("State as an Integer");
        ui.label(format!(
            "{} (big endian)",
            bits_to_int_big_endian(&self.rng.bits)
        ));
        ui.label(format!(
            "{} (little endian)",
            bits_to_int_little_endian(&self.rng.bits)
        ));

        if ui.button("step").clicked() {
            self.rng.step();
        }
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
