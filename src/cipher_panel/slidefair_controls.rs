use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::playfair::Slidefair;
use egui::Ui;
use rand::thread_rng;
use utils::{preset_alphabet::Alphabet, text_functions::shuffled_str};

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
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/playfair/slidefair.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.subheading("Alphabet");
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.cipher.assign_alphabet(&self.alphabet_string)
        }
        ui.add_space(16.0);

        ui.subheading("Keyword");
        if ui.control_string(&mut self.keyword_string).changed() {
            self.cipher.assign_key(&self.keyword_string)
        }
        ui.add_space(16.0);

        ui.subheading("Spacer Character");
        ui.label("Inserted at end if needed.");
        if ui
            .string_slider(&self.alphabet_string, &mut self.spacer_position)
            .changed()
        {
            self.cipher.spacer = self
                .alphabet_string
                .chars()
                .nth(self.spacer_position)
                .unwrap()
        }
        ui.add_space(16.0);

        ui.subheading("Grid");
        for row in self.cipher.rows() {
            ui.mono(row);
        }

        ui.add_space(16.0);
    }

    fn randomize(&mut self) {
        self.keyword_string = shuffled_str(&self.alphabet_string, &mut thread_rng());
        self.cipher.assign_key(&self.keyword_string)
    }

    crate::simple_cipher! {}
}
