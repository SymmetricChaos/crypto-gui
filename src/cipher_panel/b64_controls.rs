use ciphers::{polybius::B64, Cipher};
use egui::Ui;

use super::{CipherFrame, _generic_components::control_string};
use crate::egui_aux::mono;

#[derive(Default)]
pub struct B64Frame {
    cipher: B64,
    polybius_key_string: String,
    columnar_key_string_1: String,
    columnar_key_string_2: String,
}

impl CipherFrame for B64Frame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Polybius Key Word");
        if control_string(ui, &mut self.polybius_key_string).changed() {
            self.cipher.polybius.assign_key(&self.polybius_key_string)
        }
        ui.add_space(16.0);

        ui.label(mono(format!("Grid\n{}", self.cipher.polybius)));
        ui.add_space(16.0);

        ui.label("First Columnar Key Word");
        ui.label("Polybius Key Word");
        if control_string(ui, &mut self.columnar_key_string_1).changed() {
            self.cipher
                .columnar1
                .assign_key(&self.columnar_key_string_1)
        }
        ui.add_space(8.0);
        ui.label("Second Columnar Key Word");
        ui.label("Polybius Key Word");
        if control_string(ui, &mut self.columnar_key_string_2).changed() {
            self.cipher
                .columnar2
                .assign_key(&self.columnar_key_string_2)
        }
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
