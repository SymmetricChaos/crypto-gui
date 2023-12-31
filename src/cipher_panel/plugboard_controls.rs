use std::cmp::max;

use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{substitution::Plugboard, Cipher};
use egui::Ui;
use rand::{thread_rng, Rng};
use utils::{preset_alphabet::Alphabet, text_functions::shuffled_str};

pub struct PlugboardFrame {
    cipher: Plugboard,
    pairs: String,
    alphabet: String,
    pairs_err: String,
}

impl Default for PlugboardFrame {
    fn default() -> Self {
        let mut f = Self {
            cipher: Default::default(),
            pairs: String::from("AC EG IK MO QS UW"),
            alphabet: String::from(Alphabet::BasicLatin),
            pairs_err: String::new(),
        };
        f.cipher.set_plugboard(&f.pairs).unwrap();
        f
    }
}

impl PlugboardFrame {
    fn set_plugboard(&mut self) {
        match self.cipher.set_plugboard(&self.pairs) {
            Ok(_) => self.pairs_err.clear(),
            Err(e) => {
                self.pairs_err = e.to_string();
            }
        }
    }
}

impl CipherFrame for PlugboardFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.add_space(16.0);
        ui.subheading("Alphaabet");
        ui.control_string(&mut self.alphabet);

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.subheading("Plugboard Pairs");
            if ui.button("🎲").clicked() {
                self.randomize();
                self.set_plugboard();
            }
        });
        if ui.control_string(&mut self.pairs).changed() {
            self.set_plugboard();
        };
        ui.error_text(&self.pairs_err);

        ui.add_space(16.0);
        let nrows = 8;
        let ncols = 8;
        ui.columns(ncols, |columns| {
            let mut ctr = 0;
            let mut col = 0;
            for pair in self.cipher.show_settings() {
                columns[col].mono_strong(pair);
                ctr += 1;
                if ctr % nrows == 0 {
                    col += 1
                }
            }
        });
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        let alpha = shuffled_str(&self.alphabet, &mut rng);
        let mut cs = alpha.chars();
        let min_len = max(self.alphabet.len() / 4, 1);
        let max_len = max(self.alphabet.len() / 2, 2);
        self.pairs.clear();
        for _ in 0..rng.gen_range(min_len..max_len) {
            self.pairs.push(cs.next().unwrap());
            self.pairs.push(cs.next().unwrap());
            self.pairs.push(' ');
        }
        self.pairs.pop();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
