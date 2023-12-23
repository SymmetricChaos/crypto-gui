use super::ClassicRngFrame;
use crate::ui_elements::UiElements;
use egui::DragValue;
use rand::{thread_rng, Rng};
use rngs::{halton::HaltonSequence, ClassicRng};

pub struct HaltonFrame {
    rng: HaltonSequence,
    vector_length: usize,
    randoms: String,
}

impl Default for HaltonFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            vector_length: 2,
            randoms: String::new(),
        }
    }
}

impl HaltonFrame {}

impl ClassicRngFrame for HaltonFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Number of Dimensions");
            if ui
                .add(DragValue::new(&mut self.vector_length).clamp_range(1..=4))
                .changed()
            {
                if self.vector_length > self.rng.bases.len() {
                    let extra = self.vector_length - self.rng.bases.len();
                    self.rng.bases.extend(std::iter::once(2).take(extra));
                    self.rng.nums.extend(std::iter::once(0).take(extra));
                    self.rng.dens.extend(std::iter::once(1).take(extra));
                } else {
                    self.rng.bases.truncate(self.vector_length);
                    self.rng.nums.truncate(self.vector_length);
                    self.rng.dens.truncate(self.vector_length);
                }
            };
        });
        for b in self.rng.bases.iter_mut() {
            ui.add(DragValue::new(b).clamp_range(2..=32));
            ui.end_row();
        }

        ui.add_space(8.0);
        ui.subheading("Fractions");
        if ui.small_button("Return to Start").clicked() {
            self.rng.nums.iter_mut().for_each(|x| *x = 0);
            self.rng.dens.iter_mut().for_each(|x| *x = 1);
        }
        ui.label(format!("({:?})", self.rng.ratio_strings().join(", ")));

        ui.add_space(8.0);
        if ui.button("step").clicked() {
            self.rng.next_u32();
        }
        ui.add_space(8.0);

        if ui.button("Generate Random Tuples").clicked() {
            for _ in 0..5 {
                // This always produces zero and so is just dropped. We're just advancing the state of the rng.
                self.rng.next_u32();
                if !self.randoms.is_empty() {
                    self.randoms.push_str(", ");
                }
                self.randoms.push('(');
                self.randoms.push_str(&self.rng.ratio_strings().join(", "));
                self.randoms.push(')');
            }
        }
        ui.text_edit_multiline(&mut self.randoms);
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
