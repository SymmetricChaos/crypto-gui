use super::CipherFrame;
use crate::ui_elements::{block_cipher_mode, block_cipher_padding, UiElements};
use ciphers::{digital::block_ciphers::gost::Gost, Cipher};
use egui::Ui;
use rand::{thread_rng, Rng};

pub struct GostFrame {
    cipher: Gost,
}

impl Default for GostFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
        }
    }
}

impl CipherFrame for GostFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/tree/master/ciphers/src/digital/block_ciphers/gost.rs",
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

        ui.horizontal(|ui| {
            ui.subheading("Key");
            ui.random_bytes_button(&mut self.cipher.key);
        });
        for k in self.cipher.key.iter_mut() {
            ui.u32_drag_value_hex(k);
        }
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.subheading("Sboxes");
            ui.random_bytes_button(&mut self.cipher.sboxes);
        });
        for s in self.cipher.sboxes.iter_mut() {
            ui.u64_drag_value_hex(s);
        }

        ui.add_space(8.0);

        ui.add_enabled_ui(self.cipher.mode.iv_needed(), |ui| {
            ui.horizontal(|ui| {
                ui.subheading("IV/Counter");
                ui.random_num_button(&mut self.cipher.iv)
            });
            ui.label("In the selected mode the cipher must have a 64-bit initial value provided.");
            ui.u64_drag_value_hex(&mut self.cipher.iv);
        });

        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        rng.fill(&mut self.cipher.key);
        rng.fill(&mut self.cipher.sboxes);

        if self.cipher.mode.iv_needed() {
            self.cipher.iv = rng.gen();
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
