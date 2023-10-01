use std::collections::VecDeque;

use crypto_gui::ui_elements::UiElements;
use egui::{DragValue, RichText};
use rand::{thread_rng, Rng};
use rngs::{lfg::Lfg, ClassicRng};
use utils::bits::{bits_to_int_big_endian, bits_to_int_little_endian, Bit};

use crate::ui_elements::UiElements;

use super::ClassicRngFrame;

pub struct LfgFrame {
    rng: Lfg,
    vector_length: usize,
    state_strings: VecDeque<String>,
}

impl Default for LfgFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            vector_length: 16,
            state_strings: VecDeque::from([]),
        }
    }
}

impl LfgFrame {}

impl ClassicRngFrame for LfgFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.subheading("Vector Length");
        if ui
            .add(DragValue::new(&mut self.vector_length).clamp_range(2..=20))
            .changed()
        {
            self.rng.state.truncate(self.vector_length);
            while self.rng.state.len() < self.vector_length {
                self.rng.state.push_back(1)
            }

            self.rng.tap = self.rng.tap.min(self.rng.state.len() - 1);
        };
        ui.add_space(16.0);

        ui.subheading("State");
        ui.label("Numbers stored in the vector");

        ui.add_space(8.0);

        ui.add_space(8.0);
        if ui.button("step").clicked() {
            self.rng.step();
        }

        ui.add_space(8.0);
        ui.subheading(format!("Next Value: {}", self.rng.next_bit()));
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
