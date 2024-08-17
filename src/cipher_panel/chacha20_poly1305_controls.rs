use ciphers::digital::stream_ciphers::chacha::chacha20poly1305::ChaCha20Poly1305;
use rand::{thread_rng, Rng};

use crate::ui_elements::UiElements;

use super::CipherFrame;

pub struct ChaCha20Poly1305Frame {
    cipher: ChaCha20Poly1305,
}

impl Default for ChaCha20Poly1305Frame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
        }
    }
}

impl ChaCha20Poly1305Frame {
    fn start_state(&self) -> String {
        let mut out = String::new();

        let state = self.cipher.cipher.create_state(self.cipher.ctr);

        for line in state.chunks_exact(4) {
            for word in line {
                out.push_str(&format!("{:08x?}  ", word))
            }
            out.push('\n')
        }

        out
    }
}

impl CipherFrame for ChaCha20Poly1305Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/stream_ciphers/chacha",
        );
        ui.add_space(8.0);

        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.byte_io_mode_cipher(
            &mut self.cipher.cipher.input_format,
            &mut self.cipher.cipher.output_format,
        );

        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Key (128-bits)");
            ui.random_bytes_button(&mut self.cipher.cipher.key);
        });
        ui.horizontal(|ui| {
            for i in 0..4 {
                ui.u32_drag_value_hex(&mut self.cipher.cipher.key[i]);
            }
        });
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.subheading("Nonce (96-bits)");
            ui.random_bytes_button(&mut self.cipher.cipher.nonce);
        });
        ui.label("It is suggested that two of words of the nonce be chosen randomly for each message and the third be chosen to separate multiple streams of data.");
        ui.horizontal(|ui| {
            for i in 0..3 {
                ui.u32_drag_value_hex(&mut self.cipher.cipher.nonce[i]);
            }
        });

        ui.add_space(8.0);
        ui.subheading("Counter");
        ui.u32_drag_value_hex(&mut self.cipher.cipher.ctr);

        ui.add_space(8.0);
        ui.subheading("Number of Rounds");
        ui.label("The ChaCha20-Poly1305 standard does not accept a variant number of rounds.");

        ui.add_space(8.0);
        ui.subheading("Starting State");
        ui.label(self.start_state());
    }

    fn cipher(&self) -> &dyn ciphers::Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        rng.fill(&mut self.cipher.cipher.key);
        rng.fill(&mut self.cipher.cipher.nonce);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
