use crate::egui_aux::mono;

use super::CipherFrame;
use super::_generic_components::control_string;
use ciphers::polybius::Adfgvx;
use ciphers::traits::Cipher;
use egui::Color32;
use utils::preset_alphabet::PresetAlphabet;

#[derive(Default)]
pub struct AdfgvxFrame {
    cipher: Adfgvx,
    columnar_key_string: String,
    polybius_key_string: String,
}

impl CipherFrame for AdfgvxFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Select Mode");
        ui.horizontal(|ui| {
            if ui.button("ADFGX").clicked() {
                self.cipher
                    .polybius
                    .pick_alphabet(PresetAlphabet::BasicLatinNoJ);
                self.polybius_key_string = PresetAlphabet::BasicLatinNoJ.string();
            };
            if ui.button("ADFGVX").clicked() {
                self.cipher
                    .polybius
                    .pick_alphabet(PresetAlphabet::BasicLatinWithDigits);
                self.polybius_key_string = PresetAlphabet::BasicLatinWithDigits.string();
            };
        });

        // False alphabet display
        ui.label(mono(&self.polybius_key_string).background_color(Color32::BLACK));
        ui.add_space(16.0);

        ui.label("Polybius Key Word");
        if control_string(ui, &mut self.polybius_key_string).changed() {
            self.cipher.columnar.assign_key(&self.polybius_key_string)
        }
        ui.add_space(16.0);

        ui.add_space(16.0);
        ui.label("Grid");
        ui.label(mono(self.cipher.polybius.show_grid()));

        ui.label("Columnar Key Word");
        if control_string(ui, &mut self.columnar_key_string).changed() {
            self.cipher.columnar.assign_key(&self.columnar_key_string)
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
