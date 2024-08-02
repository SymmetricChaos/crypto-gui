use ciphers::{digital::stream_ciphers::a51::A51, Cipher};
use rand::{thread_rng, Rng};

use crate::ui_elements::UiElements;

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
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.subheading("LFSRs");
        ui.label(format!("{:#019b}", self.cipher.rng.lfsrs[0].register));
        ui.label(format!("{:#022b}", self.cipher.rng.lfsrs[1].register));
        ui.label(format!("{:#023b}", self.cipher.rng.lfsrs[2].register));

        ui.subheading("Key (Taken in Big-endian Order)");
        if ui.u64_drag_value_hex(&mut self.key).changed() {
            self.cipher
                .rng
                .ksa(self.key.to_be_bytes(), self.frame_number)
        }

        ui.subheading("Frame Number (Limited to 22 Bits)");
        if ui.u32_drag_value_hex(&mut self.frame_number).changed() {
            self.frame_number &= 0x3fffff; // mask off the high bits
            self.cipher
                .rng
                .ksa(self.key.to_be_bytes(), self.frame_number)
        }
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
        self.cipher
            .rng
            .ksa(self.key.to_be_bytes(), self.frame_number)
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
