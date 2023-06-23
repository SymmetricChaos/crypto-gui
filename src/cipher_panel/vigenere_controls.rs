use super::CipherFrame;
use crate::ui_elements::{control_string, randomize_reset};
use ciphers::{
    polyalphabetic::{PolyMode, Vigenere},
    Cipher,
};
use egui::{Slider, TextEdit, TextStyle, Ui};
use rand::{thread_rng, Rng};
use utils::{
    functions::{filter_string, random_sample_replace},
    preset_alphabet::Alphabet,
};

pub struct VigenereFrame {
    cipher: Vigenere,
    alphabet_string: String,
}

impl Default for VigenereFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatin.into(),
        }
    }
}

impl CipherFrame for VigenereFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string)
        }
        ui.add_space(16.0);
        ui.add_space(16.0);

        ui.label("Mode");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.cipher.mode, PolyMode::CylicKey, "Cyclic");
            ui.selectable_value(&mut self.cipher.mode, PolyMode::Autokey, "Autokey");
            ui.selectable_value(&mut self.cipher.mode, PolyMode::ProgKey, "Progressive");
        });

        if self.cipher.mode == PolyMode::ProgKey {
            ui.add_space(16.0);
            ui.label("Step size");
            let alpha_range = 0..=(self.cipher.alphabet_len() - 1);
            ui.add(Slider::new(&mut self.cipher.prog_shift, alpha_range));
            ui.add_space(16.0);
        }

        match self.cipher.multikey {
            true => {
                ui.horizontal(|ui| {
                    ui.label("Keywords");
                    ui.checkbox(&mut self.cipher.multikey, "Multikey");
                });
                for keyword in self.cipher.keywords.iter_mut() {
                    if control_string(ui, keyword).changed() {
                        filter_string(keyword, &self.alphabet_string)
                    }
                }
            }
            false => {
                ui.horizontal(|ui| {
                    ui.label("Keyword ");
                    ui.checkbox(&mut self.cipher.multikey, "Multikey");
                });
                ui.add(
                    TextEdit::singleline(&mut self.cipher.keywords[0]).font(TextStyle::Monospace),
                );
            }
        }
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        for keyword in self.cipher.keywords.iter_mut() {
            *keyword = random_sample_replace(&self.alphabet_string, rng.gen_range(3..12), &mut rng);
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
