use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::digital::stream_ciphers::hc256::Hc256;
use rand::{thread_rng, Rng};

pub struct Hc256Frame {
    rng: Hc256,
    key: [u32; 8],
    iv: [u32; 8],
}

impl Default for Hc256Frame {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            key: [0; 8],
            iv: [0; 8],
        }
    }
}

impl CipherFrame for Hc256Frame {
    fn ui(&mut self, ui: &mut egui::Ui, errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/digital/stream_ciphers/hc256.rs",
        );

        ui.add_space(8.0);
        ui.subheading("Key");
        for i in 0..8 {
            if ui.u32_hex_edit(&mut self.key[i]).lost_focus() {
                self.rng = Hc256::with_key_and_iv_u32(self.key, self.iv);
            }
        }

        ui.add_space(8.0);
        ui.subheading("Nonce");
        for i in 0..8 {
            if ui.u32_hex_edit(&mut self.iv[i]).lost_focus() {
                self.rng = Hc256::with_key_and_iv_u32(self.key, self.iv);
            }
        }
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        rng.fill(&mut self.key);
        rng.fill(&mut self.iv);
    }

    fn reset(&mut self) {
        todo!()
    }

    fn encrypt_string(&self, text: &str) -> Result<String, ciphers::CipherError> {
        todo!()
    }

    fn decrypt_string(&self, text: &str) -> Result<String, ciphers::CipherError> {
        todo!()
    }
}
