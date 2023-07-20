use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{
    polybius::{adfgvx::AdfgvxMode, Adfgvx},
    traits::Cipher,
};
use rand::{thread_rng, Rng};
use utils::{
    preset_alphabet::Alphabet,
    text_functions::{filter_string, shuffled_str},
};

pub struct AdfgvxFrame {
    cipher: Adfgvx,
    columnar_key_string: String,
    polybius_key_string: String,
}

impl Default for AdfgvxFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            columnar_key_string: Default::default(),
            polybius_key_string: Default::default(),
        }
    }
}

impl AdfgvxFrame {
    fn assign_columnar_key(&mut self) {
        filter_string(&mut self.columnar_key_string, &self.cipher.alphabet());
        self.cipher
            .assign_columnar_key(&self.columnar_key_string)
            .unwrap() // justified by filtering of key_string
    }
}

impl CipherFrame for AdfgvxFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.group(|ui| {
            ui.subheading("Select Mode");
            ui.horizontal(|ui| {
                if ui.button("ADFGX").clicked() {
                    self.cipher.assign_mode(AdfgvxMode::Short);
                    filter_string(&mut self.columnar_key_string, &self.cipher.alphabet());
                    filter_string(&mut self.polybius_key_string, &self.cipher.alphabet());
                    self.cipher.assign_polybius_key(&self.polybius_key_string);
                    self.assign_columnar_key();
                };
                if ui.button("ADFGVX").clicked() {
                    self.cipher.assign_mode(AdfgvxMode::Long);
                    self.cipher.assign_polybius_key(&self.polybius_key_string);
                    self.assign_columnar_key();
                };
            });
        });

        // False alphabet display
        ui.false_control_string(&self.cipher.alphabet());
        ui.add_space(16.0);

        ui.subheading("Polybius Keyword");
        if ui.control_string(&mut self.polybius_key_string).changed() {
            filter_string(&mut self.polybius_key_string, &self.cipher.alphabet());
            self.cipher.assign_polybius_key(&self.polybius_key_string)
        }

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.subheading("Polybius Grid");
            ui.copy_to_clipboard(self.cipher.show_polybius_grid());
        });
        ui.mono(self.cipher.show_polybius_grid());
        ui.add_space(8.0);

        ui.subheading("Columnar Keyword");
        if ui.control_string(&mut self.columnar_key_string).changed() {
            self.assign_columnar_key()
        }
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.polybius_key_string = shuffled_str(self.cipher.alphabet(), &mut thread_rng());
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
