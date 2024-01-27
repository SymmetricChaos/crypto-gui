use std::num::ParseIntError;

use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{
    digital::{rc5::Rc5, ByteFormat},
    Cipher,
};
use egui::{FontId, RichText, Ui};
use rand::{thread_rng, Rng};

#[derive(Default)]
pub struct Rc5Frame {
    cipher: Rc5,
    key: String,
}

impl Rc5Frame {
    fn run_ksa(&mut self) {
        let key_vec: Result<Vec<u8>, ParseIntError> = (0..self.key.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&self.key[i..i + 2], 16))
            .collect();
        if let Ok(vec) = key_vec {
            self.cipher.ksa_32(&vec)
        } else {
            unreachable!("RC5 key should be forced to valid hex digits by filtering")
        }
    }
}

impl CipherFrame for Rc5Frame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.subheading("Input Format");
        ui.label("Input can be either hexadecimal representing bytes or text, which will be interpreted as UTF-8 bytes.");
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.cipher.input_format,
                ByteFormat::Hex,
                "Hexadecimal",
            );
            ui.selectable_value(
                &mut self.cipher.input_format,
                ByteFormat::Utf8,
                "Text (UTF-8)",
            );
        });

        ui.add_space(8.0);

        ui.subheading("Output Format");
        ui.label("Output can be either hexadecimal representing the encrypted bytes or the bytes interpreted as UTF-8 encoded text. When outputing text information may be lost due to some bytes not forming valid characters.");
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.cipher.output_format,
                ByteFormat::Hex,
                "Hexadecimal",
            );
            ui.selectable_value(
                &mut self.cipher.output_format,
                ByteFormat::Utf8,
                "Text (UTF-8)",
            );
        });

        ui.add_space(16.0);

        ui.subheading("Key");
        ui.label("Key should be provided as a string of hexadecimal digits representing between 1 and 255 bytes.");
        if ui.text_edit_multiline(&mut self.key).changed() {
            self.key = self.key.chars().filter(|c| c.is_ascii_hexdigit()).collect();
        }
        if ui.button("Set State from Key").clicked() {
            if self.key.len() > 510 {
                self.key.truncate(510)
            }
            if self.key.len() % 2 == 1 {
                self.key.push('0')
            }
            self.run_ksa()
        }
        ui.add_space(16.0);

        ui.subheading("Internal State");

        ui.collapsing("Array of 32-bit Words", |ui| {
            egui::Grid::new("rc4_array")
                .num_columns(16)
                .striped(true)
                .show(ui, |ui| {
                    for (n, b) in self.cipher.state.iter().enumerate() {
                        if n % 16 == 0 && n != 0 {
                            ui.end_row()
                        }
                        ui.label(
                            RichText::from(format!("{:08X}", b)).font(FontId::monospace(15.0)),
                        );
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
        self.run_ksa();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
