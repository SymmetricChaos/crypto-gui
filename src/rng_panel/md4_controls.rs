use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_nums_box, UiElements};
use egui::DragValue;
use rand::{thread_rng, Rng};
use rngs::md4::Md4;

pub struct Md4Frame {
    rng: Md4,
    randoms: String,
    n_random: usize,
    to_hash: String,
    hashed: String,
}

impl Default for Md4Frame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            to_hash: String::from("Message digest"),
            hashed: String::new(),
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl ClassicRngFrame for Md4Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Counter");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                self.randomize();
            }
        });
        ui.add(DragValue::new(&mut self.rng.ctr));

        ui.add_space(16.0);
        ui.collapsing("Hash Text", |ui| {
            if ui.text_edit_multiline(&mut self.to_hash).changed() {
                self.hashed = format!("{:X}", Md4::hash(&self.to_hash.as_bytes()))
            }
            ui.text_edit_multiline(&mut self.hashed);
        });

        ui.add_space(16.0);
        generate_random_nums_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.rng.ctr = rng.gen::<u64>();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
