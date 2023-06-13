use super::CipherFrame;
use crate::ui_elements::{control_string, mono, randomize_reset};
use ciphers::{polybius::B64, Cipher};
use egui::Ui;
use rand::{thread_rng, Rng};
use utils::{
    functions::{random_sample_replace, shuffled_str},
    preset_alphabet::Alphabet,
};

#[derive(Default)]
pub struct B64Frame {
    cipher: B64,
    polybius_key_string: String,
    columnar_key_string_1: String,
    columnar_key_string_2: String,
}

impl CipherFrame for B64Frame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Polybius Key Word");
        if control_string(ui, &mut self.polybius_key_string).changed() {
            self.cipher.assign_polybius_key(&self.polybius_key_string)
        }
        ui.add_space(16.0);

        ui.label(mono(format!("Grid\n{}", self.cipher.polybius_grid())));
        ui.add_space(16.0);

        ui.label("First Columnar Key Word");
        ui.label("Polybius Key Word");
        if control_string(ui, &mut self.columnar_key_string_1).changed() {
            self.cipher
                .assign_columnar_key_1(&self.columnar_key_string_1)
        }
        ui.add_space(8.0);
        ui.label("Second Columnar Key Word");
        ui.label("Polybius Key Word");
        if control_string(ui, &mut self.columnar_key_string_2).changed() {
            self.cipher
                .assign_columnar_key_2(&self.columnar_key_string_2)
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
        let n_chars = rng.gen_range(6..10);
        self.columnar_key_string_1 =
            random_sample_replace(Alphabet::BasicLatin.slice(), n_chars, &mut rng);
        self.cipher
            .assign_columnar_key_1(&self.columnar_key_string_1);

        // Second columnar
        let n_chars = rng.gen_range(6..10);
        self.columnar_key_string_2 =
            random_sample_replace(Alphabet::BasicLatin.slice(), n_chars, &mut rng);
        self.cipher
            .assign_columnar_key_1(&self.columnar_key_string_2);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
