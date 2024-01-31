use std::num::ParseIntError;

use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{
    digital::{rc4::Rc4, ByteFormat},
    Cipher,
};
use egui::{DragValue, FontId, RichText, Ui};
use rand::{thread_rng, Rng};

#[derive(Default)]
pub struct Rc4Frame {
    cipher: Rc4,
    key: String,
}

impl Rc4Frame {
    fn run_ksa(&mut self) {
        let key_vec: Result<Vec<u8>, ParseIntError> = (0..self.key.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&self.key[i..i + 2], 16))
            .collect();
        if let Ok(vec) = key_vec {
            self.cipher.ksa(&vec)
        } else {
            unreachable!("RC4 key should be forced to valid hex digits by filtering")
        }
    }
}

impl CipherFrame for Rc4Frame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.collapsing("Input Format", |ui| {
            ui.label("Input can be text (interpreted as UTF-8), hexadecimal representing bytes, or Base64 representing bytes.");
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.cipher.input_format,
                    ByteFormat::Utf8,
                    "Text (UTF-8)",
                );
                ui.selectable_value(
                    &mut self.cipher.input_format,
                    ByteFormat::Hex,
                    "Hexadecimal",
                );
                ui.selectable_value(&mut self.cipher.input_format, ByteFormat::Utf8, "Base64");
            });
        });

        ui.add_space(8.0);

        ui.collapsing("Output Format", |ui| {
            ui.label("Output can be text (but information will be lost if the encrypted bytes are not valid UTF-8), hexadecimal representing bytes, or Base64 representing bytes.");
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.cipher.output_format,
                    ByteFormat::Utf8,
                    "Text (UTF-8)",
                );
                ui.selectable_value(
                    &mut self.cipher.output_format,
                    ByteFormat::Hex,
                    "Hexadecimal",
                );
                ui.selectable_value(&mut self.cipher.output_format, ByteFormat::Base64, "Base64");
            });
        });

        ui.add_space(16.0);

        ui.subheading("Key");
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
            ui.label("Pointers into the array. These always start at zero.");
            ui.horizontal(|ui| {
                ui.label("i");
                ui.add(DragValue::new(&mut self.cipher.i).clamp_range(0..=255));
            });
            ui.horizontal(|ui| {
                ui.label("j");
                ui.add(DragValue::new(&mut self.cipher.j).clamp_range(0..=255));
            });
        });
        ui.collapsing("Array of Bytes", |ui| {
            egui::Grid::new("rc4_array")
                .num_columns(16)
                .striped(true)
                .show(ui, |ui| {
                    for (n, b) in self.cipher.arr.into_iter().enumerate() {
                        if n % 16 == 0 && n != 0 {
                            ui.end_row()
                        }
                        if n == self.cipher.i as usize || n == self.cipher.j as usize {
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
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.key = format!("{:08X}", rng.gen::<u64>());
        self.cipher.i = 0;
        self.cipher.j = 0;
        self.run_ksa();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
