use std::num::ParseIntError;

use super::ClassicRngFrame;
use crate::ui_elements::{generate_random_nums_box, UiElements};
use egui::{FontId, RichText};
use rand::{thread_rng, Rng};
use rngs::mt19937_32::Mt19937_32;

pub struct MTFrame {
    rng: Mt19937_32,
    key: String,
    randoms: String,
    n_random: usize,
}

impl Default for MTFrame {
    fn default() -> Self {
        let mut rng = Mt19937_32::default();
        rng.ksa_from_array(&[0xDE_u32, 0xAD, 0xBE, 0xEF, 0x42]);
        Self {
            rng: Default::default(),
            key: String::from("DEADBEEF42"),
            randoms: String::new(),
            n_random: 5,
        }
    }
}

impl MTFrame {
    fn run_ksa(&mut self) {
        while self.key.len() % 8 != 0 {
            self.key.push('0')
        }
        let key_vec: Result<Vec<u32>, ParseIntError> = (0..self.key.len())
            .step_by(2)
            .map(|i| u32::from_str_radix(&self.key[i..i + 8], 16))
            .collect();
        if let Ok(vec) = key_vec {
            self.rng.ksa_from_array(&vec)
        } else {
            unreachable!("RC4 key should be forced to valid hex digits by filtering")
        }
    }
}

impl ClassicRngFrame for MTFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Key");
            if ui.button("🎲").on_hover_text("randomize").clicked() {
                self.randomize();
            }
        });
        ui.label("Key should be provided as a string of hexadecimal digits representing any number of bytes.");
        if ui.text_edit_multiline(&mut self.key).changed() {
            self.key = self.key.chars().filter(|c| c.is_ascii_hexdigit()).collect();
        }
        if ui.button("Set Array from Key").clicked() {
            self.run_ksa()
        }
        ui.add_space(16.0);

        ui.subheading("Internal State");
        ui.label(format!("Index: {}", self.rng.index));
        ui.collapsing("Array of 624 32-bit words", |ui| {
            egui::Grid::new("mt_array")
                .num_columns(26)
                .striped(true)
                .show(ui, |ui| {
                    for (n, b) in self.rng.arr.into_iter().enumerate() {
                        if n % 24 == 0 && n != 0 {
                            ui.end_row()
                        }
                        if n == self.rng.index as usize {
                            ui.label(
                                RichText::from(format!("{:08X}", b))
                                    .font(FontId::monospace(15.0))
                                    .strong(),
                            );
                        } else {
                            ui.label(
                                RichText::from(format!("{:08X}", b)).font(FontId::monospace(15.0)),
                            );
                        }
                    }
                });
        });

        ui.collapsing("explain", |ui| ui.label(""));

        ui.add_space(16.0);
        generate_random_nums_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
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
