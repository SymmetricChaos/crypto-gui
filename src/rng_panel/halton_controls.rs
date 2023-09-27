use super::ClassicRngFrame;
use crate::ui_elements::UiElements;
use egui::DragValue;
use rand::{thread_rng, Rng};
use rngs::{halton::HaltonSequence, ClassicRng};

pub struct HaltonFrame {
    rng: HaltonSequence,
    vector_length: usize,
}

impl Default for HaltonFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            vector_length: 2,
        }
    }
}

impl HaltonFrame {}

impl ClassicRngFrame for HaltonFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.subheading("Number of Dimensions");
        if ui
            .add(DragValue::new(&mut self.vector_length).clamp_range(1..=4))
            .changed()
        {
            self.rng.bases.truncate(self.vector_length);
            while self.rng.bases.len() < self.vector_length {
                self.rng.bases.push(2)
            }
            self.rng.nums.truncate(self.vector_length);
            while self.rng.nums.len() < self.vector_length {
                self.rng.nums.push(0)
            }
            self.rng.dens.truncate(self.vector_length);
            while self.rng.dens.len() < self.vector_length {
                self.rng.dens.push(1)
            }
        };
        ui.add_space(16.0);

        ui.subheading("Bases");
        for b in self.rng.bases.iter_mut() {
            ui.add(DragValue::new(b).clamp_range(2..=32));
            ui.end_row();
        }

        ui.add_space(8.0);
        ui.subheading("Fractions");
        if ui.small_button("Set to Start").clicked() {
            self.rng.nums.iter_mut().for_each(|x| *x = 0);
            self.rng.dens.iter_mut().for_each(|x| *x = 1);
        }
        ui.label(format!("{:?}", self.rng.ratio_strings()));

        ui.add_space(8.0);
        if ui.button("step").clicked() {
            self.rng.step();
        }
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        for base in self.rng.bases.iter_mut() {
            *base = rng.gen();
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
