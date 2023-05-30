use ciphers::{polybius::PolybiusCube, Cipher};
use egui::Ui;
use rand::thread_rng;
use utils::functions::shuffled_str;

use crate::egui_aux::mono;

use super::{
    CipherFrame,
    _generic_components::{control_string, randomize_reset},
};

pub struct PolybiusCubeFrame {
    cipher: PolybiusCube,
    alphabet_string: String,
    key_string: String,
    label_string: String,
}

impl Default for PolybiusCubeFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ+"),
            key_string: Default::default(),
            label_string: String::from("123456789"),
        }
    }
}

impl CipherFrame for PolybiusCubeFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.add_space(16.0);
        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher
                .assign_grid(&self.alphabet_string, &self.key_string)
        }

        ui.add_space(16.0);
        ui.label("Key Word");
        if control_string(ui, &mut self.key_string).changed() {
            self.cipher
                .assign_grid(&self.alphabet_string, &self.key_string)
        }

        ui.add_space(16.0);
        ui.label("Labels");
        if control_string(ui, &mut self.label_string).changed() {
            self.cipher.assign_labels(&self.label_string);
        }

        ui.add_space(16.0);
        ui.label("Grid");
        let grids = self.cipher.show_grids();
        ui.horizontal(|ui| {
            ui.label(mono(&grids[0]));
            ui.label(mono(&grids[1]));
            ui.label(mono(&grids[2]));
        });
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.key_string = shuffled_str(&self.alphabet_string, &mut thread_rng());
        self.cipher
            .assign_grid(&self.alphabet_string, &self.key_string);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
