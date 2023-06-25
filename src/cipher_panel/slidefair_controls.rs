use super::CipherFrame;
use crate::ui_elements::{control_string, mono, randomize_reset, string_slider};
use ciphers::{playfair::Slidefair, Cipher};
use egui::Ui;
use rand::thread_rng;
use utils::{functions::shuffled_str, preset_alphabet::Alphabet};

pub struct SlidefairFrame {
    cipher: Slidefair,
    alphabet_string: String,
    keyword_string: String,
    spacer_position: usize,
}

impl Default for SlidefairFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatin.into(),
            keyword_string: Default::default(),
            spacer_position: 24,
        }
    }
}

impl CipherFrame for SlidefairFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string)
        }
        ui.add_space(16.0);

        ui.label("Keyword");
        if control_string(ui, &mut self.keyword_string).changed() {
            self.cipher.assign_key(&self.keyword_string)
        }
        ui.add_space(16.0);

        ui.label("Spacer Character\nInserted at end as padding if needed");
        if string_slider(ui, &self.alphabet_string, &mut self.spacer_position).changed() {
            self.cipher.spacer = self
                .alphabet_string
                .chars()
                .nth(self.spacer_position)
                .unwrap()
        }
        ui.add_space(16.0);

        ui.label("Grid");
        for row in self.cipher.rows() {
            ui.label(mono(row));
        }

        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        self.keyword_string = shuffled_str(&self.alphabet_string, &mut thread_rng());
        self.cipher.assign_key(&self.keyword_string)
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
