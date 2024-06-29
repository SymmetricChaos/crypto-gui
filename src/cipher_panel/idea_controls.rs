use super::CipherFrame;
use crate::ui_elements::{u16_drag_value, UiElements};
use ciphers::{digital::block_ciphers::idea::Idea, Cipher};
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

impl IdeaFrame {
    // fn run_ksa(&mut self) {
    //     let key_vec: Result<Vec<u8>, ParseIntError> = (0..self.key.len())
    //         .step_by(2)
    //         .map(|i| u8::from_str_radix(&self.key[i..i + 2], 16))
    //         .collect();
    //     if let Ok(vec) = key_vec {
    //         self.cipher.ksa(&vec)
    //     } else {
    //         unreachable!("RC4 key should be forced to valid hex digits by filtering")
    //     }
    // }
}

impl CipherFrame for IdeaFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/idea.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.byte_io_mode(
            &mut self.cipher.input_format,
            &mut self.cipher.output_format,
        );

        ui.add_space(16.0);

        ui.subheading("Key");
        ui.label("They key consists of eight 16-bit words.");
        ui.horizontal(|ui| {
            for w in self.key.iter_mut() {
                if u16_drag_value(ui, w).changed() {
                    self.valid_key = false;
                }
            }
        });
        if !self.valid_key {
            self.cipher.ksa(&self.key);
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
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        rng.fill(&mut self.key);
        self.cipher.ksa(&self.key);
        self.valid_key = true;
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
