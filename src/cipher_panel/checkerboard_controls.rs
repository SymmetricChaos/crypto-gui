use crate::ui_elements::UiElements;
use ciphers::{polybius::StraddlingCheckerboard, Cipher};
use egui::{DragValue, Ui};
use rand::thread_rng;
use utils::text_functions::shuffled_str;

use super::CipherFrame;

pub struct StraddlingCheckerboardFrame {
    cipher: StraddlingCheckerboard,
    alphabet_string: String,
    top_row: String,
}

impl Default for StraddlingCheckerboardFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: String::from("ETAONRISBCDFGHJKLMPQ/UVWXYZ."),
            top_row: String::from("0123456789"),
        }
    }
}

impl CipherFrame for StraddlingCheckerboardFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/polybius/checkerboard.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);

        ui.subheading("Alphabet");
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string)
        }

        ui.subheading("Top Row");
        if ui.control_string(&mut self.top_row).changed() {
            self.cipher.assign_top_row(&self.top_row)
        }

        ui.add_space(8.0);
        let gap0 = 0..=(self.cipher.gaps.1 - 1);
        let gap1 = (self.cipher.gaps.0 + 1)..=9;

        ui.horizontal(|ui| {
            ui.mono("First Gap");
            ui.add(DragValue::new(&mut self.cipher.gaps.0).range(gap0));
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.mono("Second Gap");
            ui.add(DragValue::new(&mut self.cipher.gaps.1).range(gap1));
        });

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.subheading("Checkerboard");
            ui.copy_to_clipboard(self.cipher.cipher_page())
        });
        ui.mono(self.cipher.cipher_page());
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.alphabet_string = shuffled_str(&self.alphabet_string, &mut thread_rng());
        self.cipher.assign_alphabet(&self.alphabet_string);
        self.top_row = shuffled_str("0123456789", &mut thread_rng());
        self.cipher.assign_top_row(&self.top_row);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
