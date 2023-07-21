use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{polybius::Trifid, Cipher};
use egui::{Slider, Ui};
use rand::{thread_rng, Rng};
use utils::text_functions::shuffled_str;

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
        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.subheading("Block Size");
        ui.add(Slider::new(&mut self.cipher.block_size, 3..=30));

        ui.add_space(16.0);
        ui.subheading("Alphabet");
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.cipher
                .polybius
                .assign_grid(&self.alphabet_string, &self.key_string)
        }

        ui.add_space(16.0);
        ui.subheading("Keyword");
        if ui.control_string(&mut self.key_string).changed() {
            self.cipher
                .polybius
                .assign_grid(&self.alphabet_string, &self.key_string)
        }
        ui.add_space(16.0);

        ui.subheading("Grid");
        let grids = self.cipher.polybius.show_grids();
        ui.horizontal(|ui| {
            ui.mono(&grids[0]);
            ui.mono(&grids[1]);
            ui.mono(&grids[2]);
        });
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.cipher.block_size = thread_rng().gen_range(3..=30);
        self.key_string = shuffled_str(&self.alphabet_string, &mut thread_rng());
        self.cipher
            .polybius
            .assign_grid(&self.alphabet_string, &self.key_string);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
