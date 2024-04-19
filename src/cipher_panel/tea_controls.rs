use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{
    digital::{tea::Tea, BlockCipherMode},
    Cipher,
};
use egui::{DragValue, Ui};
use rand::{thread_rng, Rng};
use utils::byte_formatting::ByteFormat;

pub struct TeaFrame {
    cipher: Tea,
}

impl Default for TeaFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
        }
    }
}

impl TeaFrame {}

impl CipherFrame for TeaFrame {
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

        ui.collapsing("Block Cipher Mode", |ui| {
            ui.label("Input can be text (interpreted as UTF-8), hexadecimal representing bytes, or Base64 representing bytes.");
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.cipher.mode,
                    BlockCipherMode::ECB,
                    "ECB (Electronic Code Book)",
                );
                ui.selectable_value(
                    &mut self.cipher.mode,
                    BlockCipherMode::CTR,
                    "CTR (Counter)",
                );
            });
        });
        ui.add_space(8.0);

        ui.subheading("Key");
        ui.label("TEA uses four 32-bit keys or, equivalently, a single 128-bit key.");
        ui.add(DragValue::new(&mut self.cipher.key[0]).hexadecimal(8, false, true));
        ui.add(DragValue::new(&mut self.cipher.key[1]).hexadecimal(8, false, true));
        ui.add(DragValue::new(&mut self.cipher.key[2]).hexadecimal(8, false, true));
        ui.add(DragValue::new(&mut self.cipher.key[3]).hexadecimal(8, false, true));

        ui.add_space(8.0);

        ui.add_enabled_ui(self.cipher.mode == BlockCipherMode::CTR, |ui| {
            ui.subheading("Counter");
            ui.label("In CTR mode the cipher must have a 64-bit counter value provided.");
            ui.add(DragValue::new(&mut self.cipher.ctr).hexadecimal(16, false, true));
        });

        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.cipher.key[0] = rng.gen();
        self.cipher.key[1] = rng.gen();
        self.cipher.key[2] = rng.gen();
        self.cipher.key[3] = rng.gen();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
