use std::num::ParseIntError;

use super::ClassicRngFrame;
use crate::ui_elements::UiElements;
use egui::DragValue;
use rand::{thread_rng, Rng};
use rngs::{rc4::Rc4, ClassicRng};

pub struct Rc4Frame {
    rng: Rc4,
    key: String,
    random_bytes: String,
    randoms: String,
}

impl Default for Rc4Frame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            key: String::new(),
            random_bytes: String::new(),
            randoms: String::new(),
        }
    }
}

impl Rc4Frame {
    fn run_ksa(&mut self) {
        let key_vec: Result<Vec<u8>, ParseIntError> = (0..self.key.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&self.key[i..i + 2], 16))
            .collect();
        if let Ok(vec) = key_vec {
            self.rng.ksa(&vec)
        } else {
            unreachable!("RC4 key should be forced to hexdigits by filtering")
        }
    }
}

impl ClassicRngFrame for Rc4Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.subheading("Key");
        ui.label("Key should be provided as a string of hexadecimal digits.");
        if ui.text_edit_multiline(&mut self.key).changed() {
            self.key = self.key.chars().filter(|c| c.is_ascii_hexdigit()).collect();
        }
        if ui.button("Key Scheduling").clicked() {
            self.run_ksa()
        }
        ui.add_space(16.0);

        ui.subheading("Internal State");
        ui.add(DragValue::new(&mut self.rng.i).clamp_range(0..=255));
        ui.add(DragValue::new(&mut self.rng.j).clamp_range(0..=255));
        ui.collapsing("arr", |ui| {
            egui::Grid::new("rc4_array")
                .num_columns(16)
                .striped(true)
                .show(ui, |ui| {
                    for (a, b) in self.rng.arr.into_iter().enumerate() {
                        if a % 16 == 0 {
                            ui.end_row()
                        }
                        ui.mono_strong(format!("{:02X}", b));
                    }
                });
        });
        ui.add_space(16.0);
        if ui.button("step").clicked() {
            self.rng.next_byte();
        }

        ui.add_space(16.0);

        if ui.button("Random Bytes").clicked() {
            for _ in 0..5 {
                if !self.random_bytes.is_empty() {
                    self.random_bytes.push_str(", ");
                }
                self.random_bytes
                    .push_str(&format!("{:02X}", self.rng.next_byte()));
            }
        }
        ui.text_edit_multiline(&mut self.random_bytes);
        ui.add_space(16.0);

        if ui.button("Random Numbers").clicked() {
            for _ in 0..5 {
                if !self.randoms.is_empty() {
                    self.randoms.push_str(", ");
                }
                self.randoms.push_str(&self.rng.next_u32().to_string());
            }
        }
        ui.text_edit_multiline(&mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&self) -> &dyn rngs::ClassicRng {
        &self.rng
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.key = format!("{:08X}", rng.gen::<u64>());
        self.rng.i = 0;
        self.rng.j = 0;
        self.run_ksa();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
