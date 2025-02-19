use std::num::ParseIntError;

use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_u32s_box, UiElements};
use egui::{DragValue, FontId, RichText};
use rand::{thread_rng, Rng};
use rngs::{isaac::Isaac, ClassicRng};

pub struct IsaacFrame {
    rng: Isaac,
    extra_pass: bool,
    key: String,
    random_bytes: String,
    randoms: String,
    n_random_bytes: usize,
    n_random: usize,
}

impl Default for IsaacFrame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            extra_pass: true,
            key: String::from("DEADBEEF42"),
            random_bytes: String::new(),
            randoms: String::new(),
            n_random_bytes: 5,
            n_random: 5,
        }
    }
}

impl IsaacFrame {
    fn run_ksa(&mut self) {
        let key_vec: Result<Vec<u8>, ParseIntError> = (0..self.key.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&self.key[i..i + 2], 16))
            .collect();
        if let Ok(vec) = key_vec {
            self.rng.seed(&vec, self.extra_pass);
        } else {
            unreachable!("ISAAC key should be forced to valid hex digits by filtering")
        }
    }
}

impl ClassicRngFrame for IsaacFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Key");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                self.randomize();
            }
        });
        ui.label("Key should be provided as a string of hexadecimal digits representing between 1 and 256 bytes.");
        if ui.text_edit_multiline(&mut self.key).changed() {
            self.key = self.key.chars().filter(|c| c.is_ascii_hexdigit()).collect();
        }
        if ui.button("Set Array from Key").clicked() {
            if self.key.len() % 2 == 1 {
                self.key.push('0')
            }
            self.run_ksa()
        }
        ui.add_space(16.0);

        ui.subheading("Internal State");
        ui.label(format!("Output Counter: {}", self.rng.ctr));
        ui.add_space(8.0);
        ui.label("Auxiliary Variables");
        ui.label(format!("a: {:08x}", self.rng.a));
        ui.label(format!("b: {:08x}", self.rng.b));
        ui.label(format!("c: {:08x}", self.rng.c));
        ui.add_space(8.0);
        ui.collapsing("Array of State Words", |ui| {
            egui::Grid::new("isaac_array")
                .num_columns(16)
                .striped(true)
                .show(ui, |ui| {
                    for (n, b) in self.rng.array.into_iter().enumerate() {
                        if n % 16 == 0 && n != 0 {
                            ui.end_row()
                        }

                        ui.label(
                            RichText::from(format!("{:08x}", b)).font(FontId::monospace(15.0)),
                        );
                    }
                });
        });
        ui.add_space(8.0);
        ui.collapsing("Array of Output Words", |ui| {
            egui::Grid::new("isaac_output")
                .num_columns(16)
                .striped(true)
                .show(ui, |ui| {
                    for (n, b) in self.rng.array.into_iter().enumerate() {
                        if n % 16 == 0 && n != 0 {
                            ui.end_row()
                        }
                        if n == self.rng.ctr {
                            ui.label(
                                RichText::from(format!("{:08x}", b))
                                    .strong()
                                    .font(FontId::monospace(15.0)),
                            );
                        } else {
                            ui.label(
                                RichText::from(format!("{:08x}", b)).font(FontId::monospace(15.0)),
                            );
                        }
                    }
                });
        });

        ui.add_space(16.0);
        if ui.button("step").clicked() {
            self.rng.next_u32();
        }

        ui.add_space(16.0);
        generate_random_u32s_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.key = format!("{:08X}", rng.gen::<u64>());
        self.run_ksa();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
