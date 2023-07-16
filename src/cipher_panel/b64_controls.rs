use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{polybius::B64, Cipher};
use egui::Ui;
use rand::{thread_rng, Rng};
use utils::{
    preset_alphabet::Alphabet,
    text_functions::{filter_string, random_sample_replace, shuffled_str},
};

#[derive(Default)]
pub struct B64Frame {
    cipher: B64,
    polybius_key_string: String,
    columnar_key_string_1: String,
    columnar_key_string_2: String,
}

impl B64Frame {
    fn assign_columnar_key1(&mut self) {
        filter_string(&mut self.columnar_key_string_1, Alphabet::Base64.into());
        self.cipher
            .assign_columnar_key_1(&self.columnar_key_string_1)
            .unwrap()
    }

    fn assign_columnar_key2(&mut self) {
        filter_string(&mut self.columnar_key_string_2, Alphabet::Base64.into());
        self.cipher
            .assign_columnar_key_2(&self.columnar_key_string_2)
            .unwrap()
    }

    fn assign_polybius_key(&mut self) {
        filter_string(&mut self.polybius_key_string, Alphabet::Base64.into());
        self.cipher.assign_polybius_key(&self.polybius_key_string)
    }
}

impl CipherFrame for B64Frame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.subheading("Base64 Alphabet");
        ui.false_control_string(Alphabet::Base64.slice());

        ui.subheading("Polybius Keyword");
        if ui.control_string(&mut self.polybius_key_string).changed() {
            self.assign_polybius_key();
        }
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Polybius Grid");
            if ui.button("📋").on_hover_text("Copy to Clipboard").clicked() {
                ui.output_mut(|o| o.copied_text = self.cipher.polybius_grid())
            }
        });
        ui.mono(self.cipher.polybius_grid());
        ui.add_space(16.0);

        ui.subheading("First Columnar Keyword");
        if ui.control_string(&mut self.columnar_key_string_1).changed() {
            self.assign_columnar_key1()
        }
        ui.add_space(8.0);
        ui.subheading("Second Columnar Keyword");
        if ui.control_string(&mut self.columnar_key_string_2).changed() {
            self.assign_columnar_key2()
        }
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();

        // Random polybius key
        self.polybius_key_string = shuffled_str(Alphabet::Base64.slice(), &mut rng);
        self.cipher.assign_polybius_key(&self.polybius_key_string);

        // First columnar
        let n_chars = rng.gen_range(6..12);
        self.columnar_key_string_1 =
            random_sample_replace(Alphabet::BasicLatin.slice(), n_chars, &mut rng);
        self.cipher
            .assign_columnar_key_1(&self.columnar_key_string_1)
            .unwrap(); // unwrap justified by pulling from BasicLatin alphabet

        // Second columnar
        let n_chars = rng.gen_range(6..12);
        self.columnar_key_string_2 =
            random_sample_replace(Alphabet::BasicLatin.slice(), n_chars, &mut rng);
        self.cipher
            .assign_columnar_key_1(&self.columnar_key_string_2)
            .unwrap(); // unwrap justified by pulling from BasicLatin alphabet
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
