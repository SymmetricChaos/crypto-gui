use ciphers::{substitution::GeneralSubstitution, Cipher};
use egui::Ui;
use utils::preset_alphabet::PresetAlphabet;

use super::{CipherFrame, _generic_components::control_string};

pub struct GeneralSubstitutionFrame {
    cipher: GeneralSubstitution,
    pt_alphabet_string: String,
    ct_alphabet_string: String,
}

impl Default for GeneralSubstitutionFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            pt_alphabet_string: String::from(PresetAlphabet::BasicLatin),
            ct_alphabet_string: String::from("ZYXWVUTSRQPONMLKJIHGFEDCBA"),
        }
    }
}

impl CipherFrame for GeneralSubstitutionFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Plaintext Alphabet");
        if control_string(ui, &mut self.pt_alphabet_string).changed() {
            self.cipher.assign_pt_alphabet(&self.pt_alphabet_string);
        }
        ui.add_space(16.0);

        ui.label("Ciphertext Alphabet");
        if control_string(ui, &mut self.ct_alphabet_string).changed() {
            self.cipher.assign_ct_alphabet(&self.ct_alphabet_string);
        }
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
