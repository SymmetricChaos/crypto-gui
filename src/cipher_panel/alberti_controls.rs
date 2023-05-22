use crate::egui_aux::mono;

use super::{CipherFrame, _generic_components::control_string};
use ciphers::{polyalphabetic::Alberti, Cipher};
use eframe::egui::{Slider, Ui};

#[derive(Default)]
pub struct AlbertiFrame {
    cipher: Alberti,
    fixed_alphabet_string: String,
    moving_alphabet_string: String,
}

impl CipherFrame for AlbertiFrame {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Fixed Alphabet");
        if control_string(ui, &mut self.fixed_alphabet_string).changed() {
            self.cipher
                .assign_fixed_alphabet(&self.fixed_alphabet_string)
        }

        ui.label("Moving Alphabet");
        if control_string(ui, &mut self.moving_alphabet_string).changed() {
            self.cipher
                .assign_moving_alphabet(&self.moving_alphabet_string)
        }

        ui.label(mono(&self.cipher));

        ui.label("Index");
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

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
