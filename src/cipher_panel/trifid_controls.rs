use ciphers::{polybius::Trifid, Cipher};
use egui::{Slider, Ui};

use crate::egui_aux::mono;

use super::{CipherFrame, _generic_components::control_string};

pub struct TrifidFrame {
    cipher: Trifid,
    alphabet_string: String,
    key_string: String,
}

impl Default for TrifidFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ+"),
            key_string: Default::default(),
        }
    }
}

impl CipherFrame for TrifidFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        let block_size_range = 3..=30;
        ui.label("Block Size");
        ui.add(Slider::new(&mut self.cipher.block_size, block_size_range));

        ui.add_space(16.0);
        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher
                .polybius
                .define_grid(&self.alphabet_string, &self.key_string)
        }

        ui.add_space(16.0);
        ui.label("Key Word");
        if control_string(ui, &mut self.key_string).changed() {
            self.cipher
                .polybius
                .define_grid(&self.alphabet_string, &self.key_string)
        }
        ui.add_space(16.0);

        ui.label("Grid");
        let grids = self.cipher.polybius.show_grids();
        ui.horizontal(|ui| {
            ui.label(mono(&grids[0]));
            ui.label(mono(&grids[1]));
            ui.label(mono(&grids[2]));
        });
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
