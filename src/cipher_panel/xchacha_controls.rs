use ciphers::digital::stream_ciphers::chacha::{
    xchacha::XChaCha, xchacha_ietf::XChaChaIetf, ChaChaState,
};
use egui::Slider;
use rand::{thread_rng, Rng};

use crate::ui_elements::UiElements;

use super::CipherFrame;

pub struct XChaChaFrame {
    cipher: XChaCha,
    cipher_ietf: XChaChaIetf,
    ietf: bool,
}

impl Default for XChaChaFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            cipher_ietf: Default::default(),
            ietf: false,
        }
    }
}

impl XChaChaFrame {
    fn start_state(&self) -> String {
        let mut out = String::new();

        let state = if self.ietf {
            self.cipher_ietf.create_state(self.cipher_ietf.ctr)
        } else {
            self.cipher.create_state(self.cipher.ctr)
        };

        for line in state.chunks_exact(4) {
            for word in line {
                out.push_str(&format!("{:08x?}  ", word))
            }
            out.push('\n')
        }

        out
    }

    fn synthetic_key(&self) -> String {
        let mut out = String::new();

        let mut state = if self.ietf {
            ChaChaState::new([
                0x61707865,
                0x3320646e,
                0x79622d32,
                0x6b206574,
                self.cipher_ietf.key[0],
                self.cipher_ietf.key[1],
                self.cipher_ietf.key[2],
                self.cipher_ietf.key[3],
                self.cipher_ietf.key[4],
                self.cipher_ietf.key[5],
                self.cipher_ietf.key[6],
                self.cipher_ietf.key[7],
                self.cipher_ietf.nonce[0],
                self.cipher_ietf.nonce[1],
                self.cipher_ietf.nonce[2],
                self.cipher_ietf.nonce[3],
            ])
        } else {
            ChaChaState::new([
                0x61707865,
                0x3320646e,
                0x79622d32,
                0x6b206574,
                self.cipher.key[0],
                self.cipher.key[1],
                self.cipher.key[2],
                self.cipher.key[3],
                self.cipher.key[4],
                self.cipher.key[5],
                self.cipher.key[6],
                self.cipher.key[7],
                self.cipher.nonce[0],
                self.cipher.nonce[1],
                self.cipher.nonce[2],
                self.cipher.nonce[3],
            ])
        };

        out.push_str("Input\n");
        out.push_str(&state.to_string());

        for _ in 0..10 {
            state.double_round();
        }

        out.push_str("\nTransformation\n");
        out.push_str(&state.to_string());

        out.push_str("\nEncryption Key (Top and Bottom Rows)\n");
        out.push_str(&format!(
            "{:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x}",
            state[0], state[1], state[2], state[3], state[12], state[13], state[14], state[15]
        ));

        out
    }
}

impl CipherFrame for XChaChaFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/stream_ciphers/chacha",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.subheading("IETF Version");
        ui.checkbox(
            &mut self.ietf,
            "The Internet Engineering Task Force version of XChaCha uses a 32-bit counter. It does not change the size of the nonce.",
        );
        ui.add_space(8.0);

        if self.ietf {
            ui.byte_io_mode_cipher(
                &mut self.cipher_ietf.input_format,
                &mut self.cipher_ietf.output_format,
            );

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.subheading("Key (256 bits)");
                ui.random_bytes_button(&mut self.cipher_ietf.key);
            });
            ui.horizontal(|ui| {
                for i in 0..8 {
                    ui.u32_hex_edit(&mut self.cipher_ietf.key[i]);
                }
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.subheading("Nonce (192 bits)");
                ui.random_bytes_button(&mut self.cipher_ietf.nonce);
            });
            ui.label("A nonce (number used once) ensures that the cipher state is always different from message to message.");
            ui.horizontal(|ui| {
                for i in 0..2 {
                    ui.u32_hex_edit(&mut self.cipher_ietf.nonce[i]);
                }
            });

            ui.collapsing("Synthetic Key", |ui| ui.mono(self.synthetic_key()));

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.subheading("Counter (32 bits)");
                ui.random_num_button(&mut self.cipher_ietf.ctr);
            });
            ui.label("The counter ensures that each block of the keystream is different.");
            ui.u32_hex_edit(&mut self.cipher_ietf.ctr);

            ui.add_space(8.0);
            ui.subheading("Number of Rounds");
            ui.horizontal(|ui| {
                if ui.small_button("XChaCha8").clicked() {
                    self.cipher_ietf.rounds = 8;
                }
                if ui.small_button("XChaCha12").clicked() {
                    self.cipher_ietf.rounds = 12;
                }
                if ui.small_button("XChaCha20").clicked() {
                    self.cipher_ietf.rounds = 20;
                }
            });
            ui.add(Slider::new(&mut self.cipher_ietf.rounds, 2..=20));

            ui.add_space(8.0);
        } else {
            ui.byte_io_mode_cipher(
                &mut self.cipher.input_format,
                &mut self.cipher.output_format,
            );

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.subheading("Key (256 bits)");
                ui.random_bytes_button(&mut self.cipher.key);
            });
            ui.horizontal(|ui| {
                for i in 0..8 {
                    ui.u32_hex_edit(&mut self.cipher.key[i]);
                }
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.subheading("Nonce (192 bits)");
                ui.random_bytes_button(&mut self.cipher.nonce);
            });
            ui.label("A nonce (number used once) ensures that the cipher state is always different from message to message.");
            ui.horizontal(|ui| {
                for i in 0..2 {
                    ui.u32_hex_edit(&mut self.cipher.nonce[i]);
                }
            });
            ui.add_space(4.0);
            ui.collapsing("Synthetic Key", |ui| ui.label(self.synthetic_key()));

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.subheading("Counter (64 bits)");
                ui.random_num_button(&mut self.cipher.ctr);
            });
            ui.label("The counter ensures that each block of the keystream is different.");
            ui.u64_hex_edit(&mut self.cipher.ctr);

            ui.add_space(8.0);
            ui.subheading("Number of Rounds");
            ui.horizontal(|ui| {
                if ui.small_button("XChaCha8").clicked() {
                    self.cipher.rounds = 8;
                }
                if ui.small_button("XChaCha12").clicked() {
                    self.cipher.rounds = 12;
                }
                if ui.small_button("XChaCha20").clicked() {
                    self.cipher.rounds = 20;
                }
            });
            ui.add(Slider::new(&mut self.cipher.rounds, 2..=20));

            ui.add_space(8.0);
        };

        ui.subheading("Initial State");
        ui.mono(self.start_state());
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        if self.ietf {
            rng.fill(&mut self.cipher_ietf.key);
            rng.fill(&mut self.cipher_ietf.nonce);
        } else {
            rng.fill(&mut self.cipher.key);
            rng.fill(&mut self.cipher.nonce);
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }

    fn encrypt_string(&self, text: &str) -> Result<String, utils::errors::GeneralError> {
        match self.ietf {
            true => ciphers::Cipher::encrypt(&self.cipher_ietf, text),
            false => ciphers::Cipher::encrypt(&self.cipher, text),
        }
    }

    fn decrypt_string(&self, text: &str) -> Result<String, utils::errors::GeneralError> {
        match self.ietf {
            true => ciphers::Cipher::decrypt(&self.cipher_ietf, text),
            false => ciphers::Cipher::decrypt(&self.cipher, text),
        }
    }
}
