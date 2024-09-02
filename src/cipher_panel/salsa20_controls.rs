use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::digital::stream_ciphers::salsa20::Salsa20;
use egui::Slider;
use rand::{thread_rng, Rng};

pub struct Salsa20Frame {
    cipher: Salsa20,
}

impl Default for Salsa20Frame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
        }
    }
}

impl Salsa20Frame {
    fn start_state(&self) -> String {
        let mut out = String::new();

        let state = self.cipher.create_state(0);

        for line in state.chunks_exact(4) {
            for word in line {
                out.push_str(&format!("{:08x?}  ", word))
            }
            out.push('\n')
        }

        out
    }
}

impl CipherFrame for Salsa20Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/stream_ciphers/salsa20.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Key");
            ui.random_bytes_button(&mut self.cipher.key);
        });
        for i in 0..4 {
            ui.u32_hex_edit(&mut self.cipher.key[i]);
        }
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.subheading("Nonce");
            ui.random_bytes_button(&mut self.cipher.nonce);
        });
        for i in 0..2 {
            ui.u32_hex_edit(&mut self.cipher.nonce[i]);
        }
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if ui.small_button("Salsa20/8").clicked() {
                self.cipher.rounds = 8;
            }
            if ui.small_button("Salsa20/12").clicked() {
                self.cipher.rounds = 12;
            }
            if ui.small_button("Salsa20/20").clicked() {
                self.cipher.rounds = 20;
            }
        });
        ui.add(Slider::new(&mut self.cipher.rounds, 2..=20));
        ui.add_space(8.0);

        ui.subheading("Initial State");
        ui.mono(self.start_state());
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
