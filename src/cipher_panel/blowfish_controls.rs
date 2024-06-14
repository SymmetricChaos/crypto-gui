use super::CipherFrame;
use crate::ui_elements::{block_cipher_mode, UiElements};
use ciphers::{
    digital::block_ciphers::{blowfish::Blowfish, BlockCipherMode},
    Cipher,
};
use rand::{thread_rng, Rng};

pub struct BlowfishFrame {
    cipher: Blowfish,
}

impl Default for BlowfishFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
        }
    }
}

impl CipherFrame for BlowfishFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/block_ciphers/blowfish.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.byte_io_mode(
            &mut self.cipher.input_format,
            &mut self.cipher.output_format,
        );

        ui.add_space(16.0);

        block_cipher_mode(ui, &mut self.cipher.mode);
        ui.add_space(8.0);

        ui.subheading("Key");
        ui.label("Blowfish uses a key of between 4 and 72 bytes.");
        ui.horizontal(|ui| {
            if ui.small_button("-").clicked() {
                if self.cipher.key.len() > 4 {
                    self.cipher.key.pop();
                }
            }
            ui.mono(self.cipher.key.len());
            if ui.small_button("+").clicked() {
                if self.cipher.key.len() < 72 {
                    self.cipher.key.push(0)
                }
            }
        });
        for i in self.cipher.key.iter_mut() {
            ui.u8_drag_value(i)
        }

        ui.collapsing("Expanded Key", |ui| {
            ui.subheading("P-array");
            ui.label(self.cipher.parray_string());
            ui.add_space(8.0);
            ui.subheading("S-boxes");
            ui.label(self.cipher.sboxes_string());
        });

        ui.add_space(8.0);

        ui.add_enabled_ui(self.cipher.mode == BlockCipherMode::Ctr, |ui| {
            ui.subheading("Counter");
            ui.label("In CTR mode the cipher must have a 64-bit counter provided.");
            ui.u64_drag_value(&mut self.cipher.ctr);
        });

        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.cipher.key.clear();
        for _ in 0..rng.gen_range(16..=32) {
            self.cipher.key.push(rng.gen());
        }

        if self.cipher.mode == BlockCipherMode::Ctr {
            self.cipher.ctr = rng.gen();
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
