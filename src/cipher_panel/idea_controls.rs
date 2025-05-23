use super::CipherFrame;
use crate::ui_elements::{block_cipher_iv_64, block_cipher_mode_and_padding, UiElements};
use ciphers::digital::block_ciphers::idea::Idea;
use egui::{FontId, RichText, Ui};
use rand::{thread_rng, Rng};

pub struct IdeaFrame {
    cipher: Idea,
    key: [u16; 8],
    valid_key: bool,
}

impl Default for IdeaFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            key: Default::default(),
            valid_key: false,
        }
    }
}

impl CipherFrame for IdeaFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/block_ciphers/idea.rs",
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
            ui.subheading("Key (128 bits)");
            if ui.random_bytes_button(&mut self.key).clicked() {
                self.cipher.ksa_u16(self.key);
            }
        });
        ui.horizontal(|ui| {
            for w in self.key.iter_mut() {
                if ui.u16_hex_edit(w).changed() {
                    self.valid_key = false;
                }
            }
        });
        if !self.valid_key {
            self.cipher.ksa_u16(self.key);
            self.valid_key = true;
        }

        ui.collapsing("Encryption Subkeys", |ui| {
            egui::Grid::new("idea_enc_subkeys")
                .num_columns(16)
                .striped(true)
                .show(ui, |ui| {
                    for (n, w) in self.cipher.subkeys_enc().into_iter().enumerate() {
                        if n % 6 == 0 && n != 0 {
                            ui.end_row()
                        }
                        ui.label(
                            RichText::from(format!("{:04X}", w)).font(FontId::monospace(15.0)),
                        );
                    }
                });
        });

        ui.collapsing("Decryption Subkeys", |ui| {
            egui::Grid::new("idea_dec_subkeys")
                .num_columns(16)
                .striped(true)
                .show(ui, |ui| {
                    for (n, w) in self.cipher.subkeys_dec().into_iter().enumerate() {
                        if n % 6 == 0 && n != 0 {
                            ui.end_row()
                        }
                        ui.label(
                            RichText::from(format!("{:04X}", w)).font(FontId::monospace(15.0)),
                        );
                    }
                });
        });

        ui.add_space(16.0);

        block_cipher_iv_64(ui, &mut self.cipher.iv, self.cipher.mode);

        ui.add_space(16.0);
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        rng.fill(&mut self.key);
        self.cipher.ksa_u16(self.key);
        self.valid_key = true;
    }

    crate::simple_cipher! {}
}
