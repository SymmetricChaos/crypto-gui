use crate::egui_aux::mono;

use super::CipherFrame;
use ciphers::polybius::Adfgvx;
use ciphers::traits::Cipher;
use egui::TextEdit;
use utils::preset_alphabet::PresetAlphabet;

#[derive(Default)]
pub struct AdfgvxFrame {
    cipher: Adfgvx,
}

impl CipherFrame for AdfgvxFrame {
    fn ui(&mut self, ui: &mut egui::Ui, errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Select Mode");
        ui.horizontal(|ui| {
            if ui.button("ADFGX").clicked() {
                self.set_alphabet(PresetAlphabet::BasicLatinNoJ)
            };
            if ui.button("ADFGVX").clicked() {
                self.set_alphabet(PresetAlphabet::BasicLatinWithDigits)
            };
        });

        ui.label("Polybius Key Word");
        if control_string(ui, &mut self.cipher.polybius.key_word).changed() {
            self.cipher.polybius.set_key()
        }
        ui.add_space(16.0);

        ui.label(mono(format!("Grid\n{}", self.cipher.polybius)));
        ui.add_space(16.0);

        ui.label("Columnar Key Word");
        ui.add(TextEdit::singleline(self.cipher.columnar.control_key()));
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.polybius.randomize();
        self.columnar.randomize();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
