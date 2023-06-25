use super::CipherFrame;
use crate::ui_elements::{control_string, error_text, randomize_reset};
use ciphers::{
    polyalphabetic::{Quagmire, QuagmireVersion},
    Cipher,
};
use egui::{DragValue, Ui};
use rand::thread_rng;
use utils::{functions::random_sample_replace, preset_alphabet::Alphabet};

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
        randomize_reset(ui, self);
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
        ui.label("Indicator Keyword");
        if control_string(ui, &mut self.ind_key_string).changed() {
            match self.cipher.assign_ind_key(&self.ind_key_string) {
                Ok(_) => ui.label(format!("{:?}", self.cipher.ind_key())),
                Err(e) => ui.label(error_text(e.inner())),
            };
        };

        ui.add_space(8.0);
        ui.label("Indicator Letter");
        if ui
            .add(
                DragValue::new(&mut self.indicator_position)
                    .clamp_range(0..=self.alphabet_string.chars().count() - 1)
                    .custom_formatter(|n, _| {
                        let n = n as usize;
                        self.alphabet_string.chars().nth(n).unwrap().to_string()
                    })
                    .custom_parser(|s| {
                        if s.is_empty() {
                            Some(0.0)
                        } else {
                            let c = s.chars().next().unwrap();
                            self.alphabet_string
                                .chars()
                                .position(|x| x == c)
                                .map(|n| n as f64)
                        }
                    })
                    .speed(0.2),
            )
            .changed()
        {
            self.cipher.indicator = self
                .alphabet_string
                .chars()
                .nth(self.indicator_position)
                .unwrap()
        }

        ui.add_space(16.0);
        ui.label("Key #1");
        if control_string(ui, &mut self.pt_key_string).changed() {
            self.cipher.assign_pt_key(&self.pt_key_string)
        }

        if self.cipher.version == QuagmireVersion::V4 {
            ui.add_space(16.0);
            ui.label("Key #2");
            if control_string(ui, &mut self.ct_key_string).changed() {
                self.cipher.assign_ct_key(&self.ct_key_string)
            }
        }
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        self.ct_key_string = random_sample_replace(&self.alphabet_string, 9, &mut rng);
        self.pt_key_string = random_sample_replace(&self.alphabet_string, 9, &mut rng);
        self.ind_key_string = random_sample_replace(&self.alphabet_string, 9, &mut rng);
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
