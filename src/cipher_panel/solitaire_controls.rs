use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{
    polyalphabetic::{solitaire::Card, Solitaire},
    Cipher,
};
use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};

pub struct SolitaireFrame {
    cipher: Solitaire,
    keyword: String,
    example_cipher: Solitaire,
    example_keyword: String,
    unicode_names: bool,
    nth: usize,
}

impl Default for SolitaireFrame {
    fn default() -> Self {
        Self {
            cipher: Solitaire::from_keyword("CRYPTONOMICON").unwrap(),
            keyword: String::from("CRYPTONOMICON"),
            example_cipher: Solitaire::from_keyword("EXAMPLE").unwrap(),
            example_keyword: String::from("EXAMPLE"),
            unicode_names: false,
            nth: 1,
        }
    }
}

impl SolitaireFrame {
    fn display_deck(&self, ui: &mut egui::Ui, deck: &Vec<Card>) {
        if self.unicode_names {
            ui.label(deck[0..9].iter().map(|c| c.to_unicode()).join(" "));
            ui.label(deck[9..18].iter().map(|c| c.to_unicode()).join(" "));
            ui.label(deck[18..27].iter().map(|c| c.to_unicode()).join(" "));
            ui.label(deck[27..36].iter().map(|c| c.to_unicode()).join(" "));
            ui.label(deck[36..45].iter().map(|c| c.to_unicode()).join(" "));
            ui.label(deck[45..54].iter().map(|c| c.to_unicode()).join(" "));
        } else {
            ui.monospace(deck[0..9].iter().map(|c| c.to_ascii()).join(" "));
            ui.monospace(deck[9..18].iter().map(|c| c.to_ascii()).join(" "));
            ui.monospace(deck[18..27].iter().map(|c| c.to_ascii()).join(" "));
            ui.monospace(deck[27..36].iter().map(|c| c.to_ascii()).join(" "));
            ui.monospace(deck[36..45].iter().map(|c| c.to_ascii()).join(" "));
            ui.monospace(deck[45..54].iter().map(|c| c.to_ascii()).join(" "));
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

        ui.subheading("Starting Order of the Deck");
        ui.label("(jokers are XA and XB)");
        ui.add_space(4.0);
        self.display_deck(ui, &self.cipher.deck);

        ui.collapsing("Example Operations", |ui| {
            ui.subheading("Keyword");
            if ui.control_string(&mut self.example_keyword).lost_focus() {
                self.example_keyword = self
                    .example_keyword
                    .chars()
                    .filter(|c| self.example_cipher.alphabet.contains(*c)) // in case the option to change the alphabet is introduced
                    .collect();
                let _ = self.example_cipher.set_from_keyword(&self.example_keyword);
            }
            ui.subheading("Current Order of the Deck");
            ui.label("(jokers are XA and XB)");
            ui.add_space(4.0);
            self.display_deck(ui, &self.example_cipher.deck);

            ui.add_space(8.0);
            ui.label("To move the jokers first Joker A is moved one position to the right and then Joker B is moved two positions to the right.");
            if ui.button("Move Jokers").clicked() {
                self.example_cipher.move_jokers();
            }
            ui.add_space(2.0);
            ui.label("To perform a triple cut the cards to the left of the first joker are swapped with the cards to the right of the second joker.");
            if ui.button("Triple Cut").clicked() {
                self.example_cipher.triple_cut();
            }
            ui.add_space(2.0);
            ui.label("To perform a count cut the last card is used to select a position to cut. The deck is then cut at that position, though the last card is kept as the last card. This is intended to make the operation reversible.");
            if ui.button("Count Cut").clicked() {
                self.example_cipher.count_cut();
            }
            ui.add_space(2.0);
            ui.label("To perform an nth count cut a cut is made at the nth position, though the last card is kept as the last card.");
            ui.horizontal(|ui| {
                if ui.button("Nth Count Cut").clicked() {
                    self.example_cipher.count_cut_n(self.nth);
                }
                ui.add(egui::DragValue::new(&mut self.nth).range(1..=53));
            });

        });
    }

    // Unused because I'm not sure how to reverse the keyword
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
