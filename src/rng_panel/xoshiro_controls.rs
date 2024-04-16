use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, UiElements};
use rand::{thread_rng, Rng};
use rngs::xoshiro::{Scrambler, Xoshiro256};

pub struct XoshiroFrame {
    rng: Xoshiro256,
    key: [String; 4],
    randoms: String,
    n_random: usize,
}

impl Default for XoshiroFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            key: [
                String::from("0"),
                String::from("0"),
                String::from("0"),
                String::from("0"),
            ],
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl XoshiroFrame {}

impl ClassicRngFrame for XoshiroFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Seed Values");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                self.randomize();
                for i in 0..4 {
                    self.rng.state[i] = u64::from_str_radix(&self.key[i], 16).unwrap_or(0)
                }
            }
            if ui.button("set").clicked() {
                for i in 0..4 {
                    self.rng.state[i] = u64::from_str_radix(&self.key[i], 16).unwrap_or(0)
                }
            }
        });

        ui.label("Seed should be provided as four hexadecmial numbers.");

        for (i, subkey) in self.key.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("state[{}]", i));
                if ui.control_string(subkey).changed() {
                    *subkey = subkey
                        .chars()
                        .filter(|c| c.is_ascii_hexdigit())
                        .take(16)
                        .collect();
                    self.rng.state[i] = u64::from_str_radix(&subkey, 16).unwrap_or(0)
                }
            });
        }

        ui.add_space(16.0);
        ui.subheading("Scrambler");
        ui.selectable_value(&mut self.rng.scrambler, Scrambler::PlusPlus, "PlusPlus");
        ui.selectable_value(&mut self.rng.scrambler, Scrambler::StarStar, "StarStar");

        ui.collapsing("scrambler function", |ui| match self.rng.scrambler {
            Scrambler::PlusPlus => ui.label("rotate_left_23(state[0] + state[3]) + state[0]"),
            Scrambler::StarStar => ui.label("rotate_left_7(state[1] × 5) × 9"),
        });

        ui.add_space(16.0);
        ui.subheading("Steps");
        ui.label("Jump moves forward by 2^128 steps. Long jump moves forwad by 2^192 steps.");
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

        ui.add_space(16.0);
        ui.subheading("Internal State");
        ui.monospace(format!(
            "{:016X} {:016X} {:016X} {:016X}",
            self.rng.state[0], self.rng.state[1], self.rng.state[2], self.rng.state[3],
        ));

        ui.collapsing("calculations", |ui| {
            ui.label(
                "let output = scrambler_function(state)\nlet t = shift_left_17(state[1])\nstate[2] ^= state[0]\nstate[3] ^= state[1]\nstate[1] ^= state[2]\nstate[0] ^= state[3]\nstate[2] ^= t\nstate[3] = rotate_left_45(state[3])\nreturn output",
            );
        });

        ui.add_space(16.0);
        generate_random_u32s_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
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
