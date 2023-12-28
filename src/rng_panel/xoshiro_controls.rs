use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_nums_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::xoshiro::Xoshiro256;

pub struct XorshiroFrame {
    rng: Xoshiro256,
    key: String,
    randoms: String,
    n_random: usize,
}

impl Default for XorshiroFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            key: String::new(),
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl XorshiroFrame {}

impl ClassicRngFrame for XorshiroFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Seed Value");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                self.randomize();
            }
        });
        // ui.horizontal(|ui| {
        //     ui.label("Seed should be provided as a string of hexadecimal digits.");
        //     if ui.button("set").clicked() {
        //         self.rng.state = u64::from_str_radix(&self.key, 16)
        //             .expect("filtering should force this to be valid");
        //         self.set_shifts();
        //     }
        // });
        // if ui.text_edit_singleline(&mut self.key).changed() {
        //     self.key = self
        //         .key
        //         .chars()
        //         .filter(|c| c.is_ascii_hexdigit())
        //         .take(8)
        //         .collect();
        // }

        ui.add_space(16.0);
        ui.subheading("Internal State");
        ui.label("Four 64 bit words.");
        ui.label(format!(
            "{:08X} {:08X} {:08X} {:08X}",
            self.rng.state[0], self.rng.state[1], self.rng.state[2], self.rng.state[3],
        ));

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            if ui.button("step").clicked() {
                self.rng.step();
            }
            if ui.button("jump").clicked() {
                self.rng.jump();
            }
            if ui.button("long jump").clicked() {
                self.rng.long_jump();
            }
        });

        ui.collapsing("calculations", |ui| {});

        ui.add_space(16.0);
        generate_random_nums_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        for word in self.rng.state.iter_mut() {
            *word = rng.gen()
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
