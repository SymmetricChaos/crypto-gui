use ciphers::{playfair::Playfair, Cipher};
use egui::{Color32, RichText, TextEdit, TextStyle, Ui};
use utils::preset_alphabet::PresetAlphabet;

use crate::egui_aux::mono;

use super::{CipherFrame, _generic_components::control_string};

#[derive(Default)]
pub struct PlayfairFrame {
    cipher: Playfair,
    key_string: String,
}

impl CipherFrame for PlayfairFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Select Alphabet");
        ui.horizontal(|ui| {
            if ui.button("No Q").clicked() {
                self.cipher.pick_alphabet(PresetAlphabet::BasicLatinNoQ)
            };
            if ui.button("No J").clicked() {
                self.cipher.pick_alphabet(PresetAlphabet::BasicLatinNoJ)
            };
            if ui.button("Alphanumeric").clicked() {
                self.cipher
                    .pick_alphabet(PresetAlphabet::BasicLatinWithDigits)
            };
            if ui.button("Base64").clicked() {
                self.cipher.pick_alphabet(PresetAlphabet::Base64)
            };
        });

        // False alphabet block
        ui.add_space(10.0);
        ui.label(
            RichText::new(&self.cipher.alphabet)
                .monospace()
                .background_color(Color32::BLACK),
        );
        ui.add_space(16.0);

        ui.label("Key Word");
        if control_string(ui, &mut self.key_string).changed() {
            self.cipher.assign_key(&self.key_string)
        }
        ui.add_space(16.0);

        ui.label("Spacer Character\nInserted as padding where needed");
        ui.add(
            TextEdit::singleline(&mut self.cipher.control_spacer().to_string())
                .font(TextStyle::Monospace)
                .desired_width(15.0),
        );

        ui.label(mono(format!("Grid\n{}", self.cipher)));
        ui.add_space(16.0);

        //(ui, self.grid_side_len(), self.grid_side_len(), self.get_input_alphabet())
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
