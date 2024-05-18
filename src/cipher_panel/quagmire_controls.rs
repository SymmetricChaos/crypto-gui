use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{
    polyalphabetic::{Quagmire, QuagmireVersion},
    Cipher,
};
use egui::Ui;
use rand::thread_rng;
use utils::{preset_alphabet::Alphabet, text_functions::random_string_sample_replace};

pub struct QuagmireFrame {
    cipher: Quagmire,
    alphabet_string: String,
    ind_key_string: String,
    pt_key_string: String,
    ct_key_string: String,
    indicator_position: usize,
}

impl Default for QuagmireFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: String::from(Alphabet::BasicLatin),
            ind_key_string: Default::default(),
            pt_key_string: Default::default(),
            ct_key_string: Default::default(),
            indicator_position: 0,
        }
    }
}

impl CipherFrame for QuagmireFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/polyalphabetic/quagmire.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset(self);

        ui.add_space(16.0);
        ui.subheading("Alphabet");
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string)
        }

        ui.add_space(16.0);
        ui.group(|ui| {
            ui.subheading("Version");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.cipher.version, QuagmireVersion::V1, "V1");
                ui.selectable_value(&mut self.cipher.version, QuagmireVersion::V2, "V2");
                ui.selectable_value(&mut self.cipher.version, QuagmireVersion::V3, "V3");
                ui.selectable_value(&mut self.cipher.version, QuagmireVersion::V4, "V4");
            });
        });

        ui.add_space(16.0);
        ui.subheading("Indicator Keyword");
        if ui.control_string(&mut self.ind_key_string).changed() {
            match self.cipher.assign_ind_key(&self.ind_key_string) {
                Ok(_) => ui.label(format!("{:?}", self.cipher.ind_key())),
                Err(e) => ui.error_text(e),
            };
        };

        ui.add_space(8.0);
        ui.subheading("Indicator Letter");
        if ui
            .string_slider(&self.alphabet_string, &mut self.indicator_position)
            .changed()
        {
            self.cipher.indicator = self
                .alphabet_string
                .chars()
                .nth(self.indicator_position)
                .unwrap()
        }

        ui.add_space(16.0);
        ui.subheading("Key #1");
        if ui.control_string(&mut self.pt_key_string).changed() {
            self.cipher.assign_pt_key(&self.pt_key_string)
        }

        if self.cipher.version == QuagmireVersion::V4 {
            ui.add_space(16.0);
            ui.subheading("Key #2");
            if ui.control_string(&mut self.ct_key_string).changed() {
                self.cipher.assign_ct_key(&self.ct_key_string)
            }
        }
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.ct_key_string = random_string_sample_replace(&self.alphabet_string, 9, &mut rng);
        self.pt_key_string = random_string_sample_replace(&self.alphabet_string, 9, &mut rng);
        self.ind_key_string = random_string_sample_replace(&self.alphabet_string, 9, &mut rng);
        self.cipher.assign_ct_key(&self.ct_key_string);
        self.cipher.assign_pt_key(&self.pt_key_string);
        self.cipher
            .assign_ind_key(&self.ind_key_string)
            .expect("error assigning indicator");
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
