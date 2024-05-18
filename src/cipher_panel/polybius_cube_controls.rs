use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{polybius::PolybiusCube, Cipher};
use egui::Ui;
use rand::thread_rng;
use utils::text_functions::{filter_string, shuffled_str};

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
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/polybius/polybius_cube.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.subheading("Alphabet");
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.cipher
                .assign_grid(&self.alphabet_string, &self.key_string)
        }

        ui.add_space(16.0);
        ui.subheading("Keyword");
        if ui.control_string(&mut self.key_string).changed() {
            filter_string(&mut self.key_string, &self.alphabet_string);
            self.cipher
                .assign_grid(&self.alphabet_string, &self.key_string)
        }

        ui.add_space(16.0);
        ui.subheading("Labels");
        if ui.control_string(&mut self.label_string).changed() {
            self.cipher.assign_labels(&self.label_string);
        }

        ui.add_space(16.0);
        ui.subheading("Grid");
        let grids = self.cipher.show_grids();
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
        self.key_string = shuffled_str(&self.alphabet_string, &mut thread_rng());
        self.cipher
            .assign_grid(&self.alphabet_string, &self.key_string);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
