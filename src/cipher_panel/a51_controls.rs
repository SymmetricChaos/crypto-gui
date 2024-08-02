use ciphers::{digital::stream_ciphers::a51::A51, Cipher};
use rand::{thread_rng, Rng};

use super::CipherFrame;

pub struct A51Frame {
    cipher: A51,
    key: u64,
    frame_number: u32,
}

impl Default for A51Frame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            key: 0,
            frame_number: 0,
        }
    }
}

impl CipherFrame for A51Frame {
    fn ui(&mut self, ui: &mut egui::Ui, errors: &mut String) {
        todo!()
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.key = rng.gen();
        self.frame_number = rng.gen();
        self.frame_number &= 0x3fffff; // mask off the high bits
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
