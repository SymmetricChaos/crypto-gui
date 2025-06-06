use super::CipherFrame;
use crate::ui_elements::{block_cipher_iv_64, block_cipher_mode_and_padding, UiElements};
use ciphers::digital::block_ciphers::gost::Gost;
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

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.byte_io_mode_cipher(
            &mut self.cipher.input_format,
            &mut self.cipher.output_format,
        );

        ui.add_space(16.0);

        block_cipher_mode_and_padding(ui, &mut self.cipher.mode, &mut self.cipher.padding);
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.subheading("Key");
            ui.random_bytes_button(&mut self.cipher.subkeys);
        });
        for k in self.cipher.subkeys.iter_mut() {
            ui.u32_hex_edit(k);
        }
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.subheading("Sboxes");
            ui.random_bytes_button(&mut self.cipher.sboxes);
        });
        for s in self.cipher.sboxes.iter_mut() {
            ui.u64_hex_edit(s);
        }

        ui.add_space(8.0);

        block_cipher_iv_64(ui, &mut self.cipher.iv, self.cipher.mode);

        ui.add_space(16.0);
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        rng.fill(&mut self.cipher.subkeys);
        rng.fill(&mut self.cipher.sboxes);

        if self.cipher.mode.iv_needed() {
            self.cipher.iv = rng.gen();
        }
    }

    crate::simple_cipher! {}
}
