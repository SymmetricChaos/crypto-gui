use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{
    polyalphabetic::{solitaire::Card, Solitaire},
    Cipher,
};
use egui::{FontId, RichText};
use rand::{thread_rng, Rng};
use utils::preset_alphabet::Alphabet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CardFormat {
    Unicode,
    Ascii,
    Number,
}

pub struct SolitaireFrame {
    cipher: Solitaire,
    keyword: String,
    example_cipher: Solitaire,
    example_keyword: String,
    card_format: CardFormat,
    nth: usize,
}

impl Default for SolitaireFrame {
    fn default() -> Self {
        Self {
            cipher: Solitaire::from_keyword("CRYPTONOMICON").unwrap(),
            keyword: String::from("CRYPTONOMICON"),
            example_cipher: Solitaire::from_keyword("").unwrap(),
            example_keyword: String::from(""),
            card_format: CardFormat::Ascii,
            nth: 1,
        }
    }
}

impl SolitaireFrame {
    fn display_deck(&self, ui: &mut egui::Ui, deck: &Vec<Card>) {
        match self.card_format {
            CardFormat::Unicode => {
                egui::Grid::new("solitaire_array")
                    .num_columns(16)
                    .striped(true)
                    .show(ui, |ui| {
                        for (n, b) in deck.iter().enumerate() {
                            if n % 9 == 0 && n != 0 {
                                ui.end_row()
                            }
                            if b.is_joker() {
                                ui.label(
                                    RichText::from(format!("{}", b.to_unicode()))
                                        .font(FontId::monospace(15.0))
                                        .strong(),
                                );
                            } else {
                                ui.label(
                                    RichText::from(format!("{}", b.to_unicode()))
                                        .font(FontId::monospace(15.0)),
                                );
                            }
                        }
                    });
            }
            CardFormat::Ascii => {
                egui::Grid::new("solitaire_array")
                    .num_columns(16)
                    .striped(true)
                    .show(ui, |ui| {
                        for (n, b) in deck.iter().enumerate() {
                            if n % 9 == 0 && n != 0 {
                                ui.end_row()
                            }
                            if b.is_joker() {
                                ui.label(
                                    RichText::from(format!("{}", b.to_ascii()))
                                        .font(FontId::monospace(15.0))
                                        .strong(),
                                );
                            } else {
                                ui.label(
                                    RichText::from(format!("{}", b.to_ascii()))
                                        .font(FontId::monospace(15.0)),
                                );
                            }
                        }
                    });
            }
            CardFormat::Number => {
                egui::Grid::new("solitaire_array")
                    .num_columns(16)
                    .striped(true)
                    .show(ui, |ui| {
                        for (n, b) in deck.iter().enumerate() {
                            if n % 9 == 0 && n != 0 {
                                ui.end_row()
                            }
                            if b.is_joker() {
                                ui.label(
                                    RichText::from(format!("{}", b.to_string()))
                                        .font(FontId::monospace(15.0))
                                        .strong(),
                                );
                            } else {
                                ui.label(
                                    RichText::from(format!("{}", b.to_string()))
                                        .font(FontId::monospace(15.0)),
                                );
                            }
                        }
                    });
            }
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

        ui.add_space(8.0);
        ui.selectable_value(&mut self.card_format, CardFormat::Ascii, "ASCII Cards");
        ui.selectable_value(&mut self.card_format, CardFormat::Unicode, "Unicode Cards");
        ui.selectable_value(
            &mut self.card_format,
            CardFormat::Number,
            "Card Numeric Values",
        );

        ui.add_space(8.0);
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

        ui.add_space(16.0);
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
            ui.label("To move the jokers first Joker A is moved one position to the right (skipping the first position) and then Joker B is moved two positions to the right (skipping the first position).");
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
            ui.label("To perform an nth count cut a count cut is made at the nth position rather than selecting it from the last card. This is used only to arrange the deck from a keyword.s");
            ui.horizontal(|ui| {
                if ui.button("Nth Count Cut").clicked() {
                    self.example_cipher.count_cut_n(self.nth);
                }
                ui.add(egui::DragValue::new(&mut self.nth).range(1..=53));
            });

        });

        ui.add_space(16.0);
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.keyword.clear();
        for _ in 0..12 {
            let n = rng.gen_range(0..26);
            self.keyword
                .push(Alphabet::BasicLatin.chars().nth(n).unwrap());
        }
        let _ = self.cipher.set_from_keyword(&self.keyword);
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
