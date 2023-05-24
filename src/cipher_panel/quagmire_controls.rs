use ciphers::{
    polyalphabetic::{Quagmire, QuagmireVersion},
    Cipher,
};
use egui::Ui;
use utils::preset_alphabet::PresetAlphabet;

use crate::egui_aux::error_text;

use super::{CipherFrame, _generic_components::control_string};

pub struct QuagmireFrame {
    cipher: Quagmire,
    alphabet_string: String,
    ind_key_string: String,
    pt_key_string: String,
    ct_key_string: String,
}

impl Default for QuagmireFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: String::from(PresetAlphabet::BasicLatin),
            ind_key_string: Default::default(),
            pt_key_string: Default::default(),
            ct_key_string: Default::default(),
        }
    }
}

impl CipherFrame for QuagmireFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string)
        }

        ui.add_space(16.0);
        ui.label("Select Version");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.cipher.version, QuagmireVersion::V1, "V1");
            ui.selectable_value(&mut self.cipher.version, QuagmireVersion::V2, "V2");
            ui.selectable_value(&mut self.cipher.version, QuagmireVersion::V3, "V3");
            ui.selectable_value(&mut self.cipher.version, QuagmireVersion::V4, "V4");
        });

        ui.add_space(16.0);
        ui.label("Key Word");
        if control_string(ui, &mut self.ind_key_string).changed() {
            match self.cipher.assign_ind_key(&self.ind_key_string) {
                Ok(_) => ui.label(""),
                Err(e) => ui.label(error_text(e.inner())),
            };
        };

        ui.add_space(16.0);
        ui.label("Key 1");
        if control_string(ui, &mut self.pt_key_string).changed() {
            self.cipher.assign_pt_key(&self.pt_key_string)
        }

        if self.cipher.version == QuagmireVersion::V4 {
            ui.add_space(16.0);
            ui.label("Key 2");
            if control_string(ui, &mut self.ct_key_string).changed() {
                self.cipher.assign_ct_key(&self.ct_key_string)
            }
        }

        // ui.add_space(8.0);
        // ui.label(self.cipher.show_pt_key());
        // ui.add_space(8.0);
        // ui.label(self.cipher.show_ct_key());
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
