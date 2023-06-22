use crate::ui_elements::{control_string, mono, randomize_reset};

use super::CipherFrame;

use ciphers::polybius::adfgvx::AdfgvxMode;
use ciphers::polybius::Adfgvx;
use ciphers::traits::Cipher;
use egui::Color32;
use rand::{thread_rng, Rng};
use utils::functions::shuffled_str;
use utils::preset_alphabet::Alphabet;

pub struct AdfgvxFrame {
    cipher: Adfgvx,
    alphabet_string: String,
    columnar_key_string: String,
    polybius_key_string: String,
}

impl Default for AdfgvxFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatinNoJ.string(),
            columnar_key_string: Default::default(),
            polybius_key_string: Default::default(),
        }
    }
}

impl AdfgvxFrame {
    fn assign_columnar_key(&mut self) {
        self.columnar_key_string = self
            .columnar_key_string
            .chars()
            .filter(|c| !self.alphabet_string.contains(*c))
            .collect();
        self.cipher
            .assign_columnar_key(&self.columnar_key_string)
            .unwrap() // justified by filtering of key_string
    }
}

impl CipherFrame for AdfgvxFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Select Mode");
        ui.horizontal(|ui| {
            if ui.button("ADFGX").clicked() {
                self.cipher.assign_mode(AdfgvxMode::Short);
                self.alphabet_string = Alphabet::BasicLatinNoJ.string();
                self.cipher.assign_polybius_key(&self.polybius_key_string);
                self.assign_columnar_key();
            };
            if ui.button("ADFGVX").clicked() {
                self.cipher.assign_mode(AdfgvxMode::Long);
                self.alphabet_string = Alphabet::BasicLatinWithDigits.string();
                self.cipher.assign_polybius_key(&self.polybius_key_string);
                self.assign_columnar_key();
            };
        });

        // False alphabet display
        ui.label(mono(&self.alphabet_string).background_color(Color32::BLACK));
        ui.add_space(16.0);

        ui.label("Polybius Keyword");
        if control_string(ui, &mut self.polybius_key_string).changed() {
            self.cipher.assign_polybius_key(&self.polybius_key_string)
        }
        ui.add_space(16.0);

        ui.add_space(16.0);
        ui.label("Grid");
        ui.label(mono(self.cipher.show_polybius_grid()));

        ui.label("Columnar Keyword");
        if control_string(ui, &mut self.columnar_key_string).changed() {
            self.assign_columnar_key()
        }
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.polybius_key_string = shuffled_str(&self.alphabet_string, &mut thread_rng());
        self.cipher.assign_polybius_key(&self.polybius_key_string);

        let n_chars = thread_rng().gen_range(6..10);

        self.columnar_key_string.clear();
        for _ in 0..n_chars {
            self.columnar_key_string.push(
                Alphabet::BasicLatin
                    .chars()
                    .nth(thread_rng().gen_range(0..26))
                    .unwrap(),
            )
        }

        self.assign_columnar_key();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
