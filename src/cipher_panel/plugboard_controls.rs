use ciphers::{substitution::Plugboard, Cipher};
use egui::Ui;

use crate::egui_aux::{error_text, mono_strong};

use super::{CipherFrame, _generic_components::control_string};

#[derive(Default)]
pub struct PlugboardFrame {
    cipher: Plugboard,
    pairs: String,
}

impl CipherFrame for PlugboardFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.add_space(16.0);
        ui.label("Plugboard Pairs");
        if control_string(ui, &mut self.pairs).changed() {
            match self.cipher.set_plugboard(&self.pairs) {
                Ok(_) => (),
                Err(e) => {
                    ui.label(error_text(&e.inner()));
                }
            }
        };

        let nrows = 8;
        let ncols = 8;
        ui.columns(ncols, |columns| {
            let mut ctr = 0;
            let mut col = 0;
            for pair in self.cipher.show_settings() {
                columns[col].label(mono_strong(pair));
                ctr += 1;
                if ctr % nrows == 0 {
                    col += 1
                }
            }
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
