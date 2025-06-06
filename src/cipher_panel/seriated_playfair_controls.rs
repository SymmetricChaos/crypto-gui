use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::playfair::seriated_playfair::SeriatedPlayfair;
use egui::{Slider, Ui};
use rand::{rngs::StdRng, SeedableRng};
use utils::{
    preset_alphabet::Alphabet,
    text_functions::{filter_string, shuffled_str},
};

pub struct SeriatedPlayfairFrame {
    cipher: SeriatedPlayfair,
    key_string: String,
    alphabet_string: String,
}

impl Default for SeriatedPlayfairFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            key_string: Default::default(),
            alphabet_string: Alphabet::BasicLatinNoQ.into(),
        }
    }
}

impl CipherFrame for SeriatedPlayfairFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/playfair/seriated_playfair.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.group(|ui| {
            ui.subheading("Common Alphabets");
            ui.horizontal(|ui| {
                for (name, alphabet) in [
                    ("No C", Alphabet::BasicLatinNoC),
                    ("No J", Alphabet::BasicLatinNoJ),
                    ("No Q", Alphabet::BasicLatinNoQ),
                    ("Alphanumeric", Alphabet::Alphanumeric),
                    ("Base64", Alphabet::Base64),
                ] {
                    if ui.button(name).clicked() {
                        self.alphabet_string = alphabet.into();
                        filter_string(&mut self.key_string, &self.alphabet_string);
                        self.cipher
                            .playfair
                            .assign_key(&self.key_string, &self.alphabet_string)
                    }
                }
            });
        });

        ui.add_space(10.0);

        ui.label("Alphabet");
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.cipher
                .playfair
                .assign_key(&self.key_string, &self.alphabet_string);
        }
        ui.add_space(16.0);

        ui.label("Keyword");
        if ui.control_string(&mut self.key_string).changed() {
            self.cipher
                .playfair
                .assign_key(&self.key_string, &self.alphabet_string)
        }
        ui.add_space(16.0);

        ui.label("Period");
        ui.add(Slider::new(&mut self.cipher.period, 4..=12));

        // ui.menu_button("Spacer Character", |ui| {
        //     for c in self.alphabet_string.chars() {
        //         if ui.button(c.to_string()).clicked() {
        //             self.cipher.spacer = c
        //         }
        //     }
        // });
        // ui.label(self.cipher.spacer.to_string());

        ui.horizontal(|ui| {
            ui.label("Grid");
            ui.copy_to_clipboard(self.cipher.playfair.to_string());
        });
        ui.mono(self.cipher.playfair.to_string());
        ui.add_space(16.0);
    }

    fn randomize(&mut self) {
        self.key_string = shuffled_str(&self.cipher.playfair.square, &mut StdRng::from_entropy());
        self.cipher
            .playfair
            .assign_key(&self.key_string, &self.alphabet_string);
    }

    crate::simple_cipher! {}
}
