use crate::egui_aux::mono;

use super::CipherFrame;
use super::_generic_components::control_string;
use ciphers::polybius::adfgvx::AdfgvxMode;
use ciphers::polybius::Adfgvx;
use ciphers::traits::Cipher;
use egui::Color32;

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
                self.cipher.assign_mode(AdfgvxMode::Short);
                self.cipher.assign_polybius_key(&self.polybius_key_string);
                self.cipher.assign_columnar_key(&self.columnar_key_string);
            };
            if ui.button("ADFGVX").clicked() {
                self.cipher.assign_mode(AdfgvxMode::Long);
                self.cipher.assign_polybius_key(&self.polybius_key_string);
                self.cipher.assign_columnar_key(&self.columnar_key_string);
            };
        });

        // False alphabet display
        ui.label(mono(&self.polybius_key_string).background_color(Color32::BLACK));
        ui.add_space(16.0);

        ui.label("Polybius Key Word");
        if control_string(ui, &mut self.polybius_key_string).changed() {
            self.cipher.assign_polybius_key(&self.polybius_key_string)
        }
        ui.add_space(16.0);

        ui.add_space(16.0);
        ui.label("Grid");
        ui.label(mono(self.cipher.show_polybius_grid()));

        ui.label("Columnar Key Word");
        if control_string(ui, &mut self.columnar_key_string).changed() {
            self.cipher.assign_columnar_key(&self.columnar_key_string)
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
