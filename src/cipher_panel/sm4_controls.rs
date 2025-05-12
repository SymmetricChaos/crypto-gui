use super::CipherFrame;
use crate::ui_elements::{
    block_cipher_iv_128, block_cipher_mode, block_cipher_padding, UiElements,
};
use ciphers::{digital::block_ciphers::sm4::Sm4, Cipher};
use egui::{FontId, RichText, Ui};
use rand::{thread_rng, Rng};

pub struct Sm4Frame {
    cipher: Sm4,
    key: [u32; 4],
}

impl Default for Sm4Frame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            key: Default::default(),
        }
    }
}

impl CipherFrame for Sm4Frame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/sm4.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
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
            ui.subheading("Key (128 bits)");
            if ui.random_bytes_button(&mut self.key).clicked() {
                self.cipher.ksa_32(self.key);
            }
        });
        ui.horizontal(|ui| {
            for i in 0..4 {
                if ui.u32_hex_edit(&mut self.key[i]).changed() {
                    self.cipher.ksa_32(self.key);
                }
            }
        });

        ui.collapsing("Subkeys", |ui| {
            egui::Grid::new("sm4_subkeys")
                .num_columns(16)
                .striped(true)
                .show(ui, |ui| {
                    for (n, w) in self.cipher.subkeys.into_iter().enumerate() {
                        if n % 4 == 0 && n != 0 {
                            ui.end_row()
                        }
                        ui.label(
                            RichText::from(format!("{:08X}", w)).font(FontId::monospace(15.0)),
                        );
                    }
                });
        });

        ui.add_space(16.0);

        block_cipher_iv_128(ui, &mut self.cipher.iv, self.cipher.mode);

        ui.add_space(16.0);
    }


    fn randomize(&mut self) {
        let mut rng = thread_rng();
        rng.fill(&mut self.key);
        self.cipher.ksa_32(self.key);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
