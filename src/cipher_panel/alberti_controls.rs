use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{polyalphabetic::Alberti, Cipher};
use eframe::egui::{Slider, Ui};
use rand::{thread_rng, Rng};
use utils::preset_alphabet::Alphabet;

pub struct AlbertiFrame {
    cipher: Alberti,
    fixed_alphabet_string: String,
    moving_alphabet_string: String,
}

impl Default for AlbertiFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            fixed_alphabet_string: String::from(Alphabet::BasicLatin),
            moving_alphabet_string: String::from(Alphabet::BasicLatin).to_ascii_lowercase(),
        }
    }
}

impl CipherFrame for AlbertiFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.subheading("Fixed Alphabet");
        if ui.control_string(&mut self.fixed_alphabet_string).changed() {
            self.cipher
                .assign_fixed_alphabet(&self.fixed_alphabet_string)
        }

        ui.subheading("Moving Alphabet");
        if ui
            .control_string(&mut self.moving_alphabet_string)
            .changed()
        {
            self.cipher
                .assign_moving_alphabet(&self.moving_alphabet_string)
        }

        ui.mono(&self.cipher);

        ui.subheading("Index");
        let alpha_range = 0..=(self.cipher.alphabet_len() - 1);
        ui.add(Slider::new(
            &mut self.cipher.start_index,
            alpha_range.clone(),
        ));
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let length = self.cipher.moving_alphabet.len();
        self.cipher.start_index = thread_rng().gen_range(0..length);
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
