use crate::ui_elements::UiElements;

use super::CipherFrame;
use ciphers::{polyalphabetic::Solitaire, Cipher};
use rand::{seq::SliceRandom, thread_rng};
use utils::{preset_alphabet::Alphabet, vecstring::VecString};

pub struct SolitaireFrame {
    cipher: Solitaire,
    keyword: String,
}

impl Default for SolitaireFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            keyword: String::from("Solitaire"),
        }
    }
}

impl CipherFrame for SolitaireFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/polyalphabetic/solitaire.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        todo!()
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.cipher.deck.shuffle(&mut rng);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }

    fn encrypt_string(&self, text: &str) -> Result<String, ciphers::CipherError> {
        self.cipher.encrypt(text)
    }

    fn decrypt_string(&self, text: &str) -> Result<String, ciphers::CipherError> {
        self.cipher.encrypt(text)
    }
}
