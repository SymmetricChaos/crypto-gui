use std::num::ParseIntError;

use super::ClassicRngFrame;
use crate::ui_elements::{generate_randoms_box, UiElements};
use egui::{DragValue, FontId, RichText};
use rand::{thread_rng, Rng};
use rngs::rc4::Rc4;

pub struct Rc4Frame {
    rng: Rc4,
    key: String,
    random_bytes: String,
    randoms: String,
    n_random_bytes: usize,
    n_random: usize,
}

impl Default for Rc4Frame {
    fn default() -> Self {
        let mut rng = Rc4::default();
        rng.ksa(&[0xDE_u8, 0xAD, 0xBE, 0xEF, 0x42]);
        Self {
            rng,
            key: String::from("DEADBEEF42"),
            random_bytes: String::new(),
            randoms: String::new(),
            n_random_bytes: 5,
            n_random: 5,
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
            unreachable!("RC4 key should be forced to valid hex digits by filtering")
        }
    }
}

impl ClassicRngFrame for Rc4Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/rngs/src/rc4.rs",
        );
        ui.add_space(8.0);

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
        if ui.button("Set Byte Array from Key").clicked() {
            if self.key.len() % 2 == 1 {
                self.key.push('0')
            }
            self.run_ksa()
        }
        ui.add_space(16.0);

        ui.subheading("Internal State");
        ui.group(|ui| {
            ui.label("Pointers into the Array");
            ui.horizontal(|ui| {
                ui.label("i");
                ui.add(DragValue::new(&mut self.rng.i).range(0..=255));
            });
            ui.horizontal(|ui| {
                ui.label("j");
                ui.add(DragValue::new(&mut self.rng.j).range(0..=255));
            });
        });
        ui.collapsing("Array of Bytes", |ui| {
            egui::Grid::new("rc4_array")
                .num_columns(16)
                .striped(true)
                .show(ui, |ui| {
                    for (n, b) in self.rng.arr.into_iter().enumerate() {
                        if n % 16 == 0 && n != 0 {
                            ui.end_row()
                        }
                        if n == self.rng.i as usize || n == self.rng.j as usize {
                            ui.label(
                                RichText::from(format!("{:02X}", b))
                                    .font(FontId::monospace(15.0))
                                    .strong(),
                            );
                        } else {
                            ui.label(
                                RichText::from(format!("{:02X}", b)).font(FontId::monospace(15.0)),
                            );
                        }
                    }
                });
        });

        ui.add_space(16.0);
        if ui.button("step").clicked() {
            self.rng.next_byte();
        }
        ui.collapsing("explain", |ui| {
            ui.label("To generate a value just two kinds of operations are used, addition of bytes and swapping of array elements. Note that addition \"wraps around\" after 255, this is also known as addition modulo 256. The procedure is as follows:\n\nAdd 1 to 'i'. Add the value that 'i' points to, to 'j'. Swap the values that 'i' and 'j' point to. Add the values that 'i' and 'j' point to. The byte at this location is the output.\n\nThis can be written more compactly. Here the notation A[x] means the byte at positionx in the array.\n\ni = i + 1\nj = j + A[i]\nswap A[i] with A[j]\nt = A[i] + A[j] (create a temporary value)\noutput A[t]")
        });

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            if ui.button("Random Bytes").clicked() {
                for _ in 0..self.n_random_bytes {
                    if !self.random_bytes.is_empty() {
                        self.random_bytes.push_str(", ");
                    }
                    self.random_bytes
                        .push_str(&format!("{:02X}", self.rng.next_byte()));
                }
            }
            ui.add(DragValue::new(&mut self.n_random_bytes).range(1..=10))
        });
        ui.text_edit_multiline(&mut self.random_bytes);

        ui.add_space(16.0);
        generate_randoms_box(ui, &mut self.rng, &mut self.n_random, &mut self.randoms);
        ui.add_space(16.0);
    }

    fn rng(&mut self) -> &mut dyn rngs::ClassicRng {
        &mut self.rng
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
