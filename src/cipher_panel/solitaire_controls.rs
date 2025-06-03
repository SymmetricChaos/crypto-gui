use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{polyalphabetic::Solitaire, Cipher};
use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};

pub struct SolitaireFrame {
    cipher: Solitaire,
    keyword: String,
}

impl Default for SolitaireFrame {
    fn default() -> Self {
        Self {
            cipher: Solitaire::from_keyword("CRYPTONOMICON").unwrap(),
            keyword: String::from("CRYPTONOMICON"),
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

        if ui.button("Reset").clicked() {
            self.reset()
        }

        ui.add_space(16.0);

        ui.subheading("Keyword");
        if ui.control_string(&mut self.keyword).lost_focus() {
            self.keyword = self
                .keyword
                .chars()
                .filter(|c| self.cipher.alphabet.contains(*c)) // in case the option to change the alphabet is introduced
                .collect();
            let _ = self.cipher.set_from_keyword(&self.keyword);
        }

        ui.subheading("Order of the Deck");
        ui.label("(jokers are XA and XB)");
        ui.add_space(4.0);
        ui.monospace(
            self.cipher.deck[0..9]
                .iter()
                .map(|c| c.to_unicode())
                .join(" "),
        );
        ui.monospace(
            self.cipher.deck[9..18]
                .iter()
                .map(|c| c.to_unicode())
                .join(" "),
        );
        ui.monospace(
            self.cipher.deck[18..27]
                .iter()
                .map(|c| c.to_unicode())
                .join(" "),
        );
        ui.monospace(
            self.cipher.deck[27..36]
                .iter()
                .map(|c| c.to_unicode())
                .join(" "),
        );
        ui.monospace(
            self.cipher.deck[36..45]
                .iter()
                .map(|c| c.to_unicode())
                .join(" "),
        );
        ui.monospace(
            self.cipher.deck[45..54]
                .iter()
                .map(|c| c.to_unicode())
                .join(" "),
        );
    }

    // Unused
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
