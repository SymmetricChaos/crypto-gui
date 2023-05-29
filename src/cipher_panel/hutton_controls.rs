use ciphers::{
    polyalphabetic::{Hutton, HuttonVersion},
    Cipher,
};
use egui::Ui;
use rand::thread_rng;
use utils::{functions::shuffled_str, preset_alphabet::Alphabet};

use super::{CipherFrame, _generic_components::control_string};

pub struct HuttonFrame {
    cipher: Hutton,
    alphabet_string: String,
    password_string: String,
    key_string: String,
}

impl Default for HuttonFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatin.into(),
            password_string: Default::default(),
            key_string: Default::default(),
        }
    }
}

impl CipherFrame for HuttonFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string)
        }

        ui.add_space(16.0);
        ui.label("Select Version");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.cipher.version, HuttonVersion::V1, "V1");
            ui.selectable_value(&mut self.cipher.version, HuttonVersion::V2, "V2");
        });

        ui.add_space(16.0);
        ui.label("Password");
        if control_string(ui, &mut self.password_string).changed() {
            self.cipher.assign_password(&self.password_string)
        }

        ui.add_space(16.0);
        ui.label("Key Word");
        if control_string(ui, &mut self.key_string).changed() {
            self.cipher.assign_key(&self.key_string)
        }
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.password_string = shuffled_str(&self.alphabet_string, &mut rng);
        self.cipher.assign_password(&self.password_string);
        self.key_string = shuffled_str(&self.alphabet_string, &mut rng);
        self.cipher.assign_key(&self.key_string);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
