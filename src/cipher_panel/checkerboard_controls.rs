use crate::ui_elements::control_string;
use crate::ui_elements::mono;
use crate::ui_elements::randomize_reset;
use crate::ui_elements::subheading;
use ciphers::{polybius::StraddlingCheckerboard, Cipher};
use egui::{DragValue, Ui};
use rand::thread_rng;
use utils::text_functions::shuffled_str;

use super::CipherFrame;

pub struct StraddlingCheckerboardFrame {
    cipher: StraddlingCheckerboard,
    alphabet_string: String,
}

impl Default for StraddlingCheckerboardFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: String::from("ETAONRISBCDFGHJKLMPQ/UVWXYZ."),
        }
    }
}

impl CipherFrame for StraddlingCheckerboardFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);

        ui.label(subheading("Alphabet"));
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string)
        }

        ui.add_space(8.0);
        let gap0 = 0..=(self.cipher.gaps.1 - 1);
        let gap1 = (self.cipher.gaps.0 + 1)..=9;

        ui.horizontal(|ui| {
            ui.label(mono("First Gap"));
            ui.add(DragValue::new(&mut self.cipher.gaps.0).clamp_range(gap0));
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label(mono("Second Gap"));
            ui.add(DragValue::new(&mut self.cipher.gaps.1).clamp_range(gap1));
        });

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.label(subheading("Checkerboard"));
            if ui.button("📋").on_hover_text("Copy to Clipboard").clicked() {
                ui.output_mut(|o| o.copied_text = self.cipher.cipher_page())
            }
        });
        ui.label(mono(self.cipher.cipher_page()).size(15.0));
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.alphabet_string = shuffled_str(&self.alphabet_string, &mut thread_rng());
        self.cipher.assign_alphabet(&self.alphabet_string)
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
