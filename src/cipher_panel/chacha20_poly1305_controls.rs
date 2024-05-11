use ciphers::digital::chacha20poly1305::ChaCha20Poly1305;
use egui::DragValue;
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
        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.subheading("Key");
        for i in 0..4 {
            ui.add(DragValue::new(&mut self.cipher.cipher.key[i]).hexadecimal(8, false, true));
        }
        ui.add_space(8.0);
        ui.subheading("Nonce");
        ui.label("The variant of ChaCha20-Poly1305 presented here uses a 96-bit nonce, consisting of three 32-bit words. The internal counter is thus only 32-bits.");
        ui.label("It is suggested that two of words of the nonce be chosen randomly for each message and the third be chosen to separate multiple streams of data.");
        for i in 0..3 {
            ui.add(DragValue::new(&mut self.cipher.cipher.nonce[i]).hexadecimal(8, false, true));
        }

        ui.add_space(8.0);
        ui.subheading("Counter");
        ui.add(DragValue::new(&mut self.cipher.cipher.ctr).hexadecimal(8, false, true));

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
