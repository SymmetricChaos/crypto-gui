use super::CipherFrame;
use crate::ui_elements::{block_cipher_mode, block_cipher_padding, UiElements};
use ciphers::{
    digital::block_ciphers::{block_cipher::BCMode, xtea::Xtea},
    Cipher,
};
use egui::Ui;
use rand::{thread_rng, Rng};

pub struct XteaFrame {
    cipher: Xtea,
}

impl Default for XteaFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
        }
    }
}

impl XteaFrame {}

impl CipherFrame for XteaFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/block_ciphers/xtea.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.byte_io_mode_cipher(
            &mut self.cipher.input_format,
            &mut self.cipher.output_format,
        );

        ui.add_space(16.0);

        block_cipher_mode(ui, &mut self.cipher.mode);
        ui.add_space(4.0);
        block_cipher_padding(ui, &mut self.cipher.padding);
        ui.add_space(8.0);

        ui.subheading("Key");
        ui.label("XTEA uses four 32-bit keys or, equivalently, a single 128-bit key.");
        for i in 0..4 {
            ui.u32_drag_value_hex(&mut self.cipher.key[i]);
        }

        ui.add_space(8.0);

        if self.cipher.mode.iv_needed() {
            if self.cipher.mode == BCMode::Ctr {
                ui.subheading("Counter");
            } else {
                ui.subheading("Initialization Vector");
            }
        } else {
            ui.subheading("Counter/IV Not Needed");
        }

        ui.add_enabled_ui(self.cipher.mode.iv_needed(), |ui| {
            ui.u64_drag_value_hex(&mut self.cipher.iv);
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

        if self.cipher.mode.iv_needed() {
            self.cipher.iv = rng.gen();
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
