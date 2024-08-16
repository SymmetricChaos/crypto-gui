use ciphers::digital::stream_ciphers::chacha::chacha_extended_nonce::ChaChaExtendedNonce;
use egui::Slider;
use rand::{thread_rng, Rng};

use crate::ui_elements::UiElements;

use super::CipherFrame;

pub struct ChaChaExNonceFrame {
    cipher: ChaChaExtendedNonce,
}

impl Default for ChaChaExNonceFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
        }
    }
}

impl ChaChaExNonceFrame {
    fn start_state(&self) -> String {
        let mut out = String::new();

        let state = self.cipher.create_state(self.cipher.ctr);

        for line in state.chunks_exact(4) {
            for word in line {
                out.push_str(&format!("{:08x?}  ", word))
            }
            out.push('\n')
        }

        out
    }
}

impl CipherFrame for ChaChaExNonceFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/stream_ciphers/chacha",
        );
        ui.add_space(8.0);

        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.byte_io_mode_cipher(
            &mut self.cipher.input_format,
            &mut self.cipher.output_format,
        );

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.subheading("Key (128-bits)");
            ui.fill_random_bytes_button(&mut self.cipher.key);
        });
        ui.horizontal(|ui| {
            for i in 0..4 {
                ui.u32_drag_value_hex(&mut self.cipher.key[i]);
            }
        });

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.subheading("Nonce (96-bits)");
            ui.fill_random_bytes_button(&mut self.cipher.nonce);
        });
        ui.label("A nonce (number used once) ensures that the cipher state is always different from message to message.");
        ui.horizontal(|ui| {
            for i in 0..3 {
                ui.u32_drag_value_hex(&mut self.cipher.nonce[i]);
            }
        });

        ui.add_space(8.0);
        ui.subheading("Counter (32-bits)");
        ui.label("The counter ensures that each block of the keystream is different.");
        ui.u32_drag_value_hex(&mut self.cipher.ctr);

        ui.add_space(8.0);
        ui.subheading("Number of Rounds");
        ui.horizontal(|ui| {
            if ui.small_button("ChaCha8").clicked() {
                self.cipher.rounds = 8;
            }
            if ui.small_button("ChaCha12").clicked() {
                self.cipher.rounds = 12;
            }
            if ui.small_button("ChaCha20").clicked() {
                self.cipher.rounds = 20;
            }
        });
        ui.add(Slider::new(&mut self.cipher.rounds, 2..=20));

        ui.add_space(8.0);

        ui.subheading("Initial State");
        ui.label(self.start_state());
    }

    fn cipher(&self) -> &dyn ciphers::Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();

        rng.fill(&mut self.cipher.key);
        rng.fill(&mut self.cipher.nonce);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
