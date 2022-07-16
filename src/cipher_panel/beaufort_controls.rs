use crate::ciphers::polyalphabetic::{Beaufort, PolyMode};

use super::{View, ViewableCipher, _generic_components::*};
use eframe::egui::{Slider, TextEdit, TextStyle, Ui};

impl ViewableCipher for Beaufort {}

impl View for Beaufort {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.set_alphabet()
        }
        ui.add_space(10.0);

        ui.label("Mode");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mode, PolyMode::CylicKey, "Cyclic");
            ui.selectable_value(&mut self.mode, PolyMode::Autokey, "Autokey");
            ui.selectable_value(&mut self.mode, PolyMode::ProgKey, "Progressive");
        });

        if self.mode == PolyMode::ProgKey {
            ui.add_space(16.0);
            ui.label("Step size");
            let alpha_range = 0..=(self.alphabet_len() - 1);
            ui.add(Slider::new(&mut self.prog_shift, alpha_range));
            ui.add_space(16.0);
        }

        match self.multikey {
            true => {
                ui.horizontal(|ui| {
                    ui.label("Key Words");
                    ui.checkbox(&mut self.multikey, "Multikey");
                });
                ui.add(TextEdit::singleline(&mut self.key_words[0]).font(TextStyle::Monospace));
                ui.add(TextEdit::singleline(&mut self.key_words[1]).font(TextStyle::Monospace));
                ui.add(TextEdit::singleline(&mut self.key_words[2]).font(TextStyle::Monospace));
                ui.add(TextEdit::singleline(&mut self.key_words[3]).font(TextStyle::Monospace));
                ui.add(TextEdit::singleline(&mut self.key_words[4]).font(TextStyle::Monospace));
            }
            false => {
                ui.horizontal(|ui| {
                    ui.label("Key Word ");
                    ui.checkbox(&mut self.multikey, "Multikey");
                });
                ui.add(TextEdit::singleline(&mut self.key_words[0]).font(TextStyle::Monospace));
            }
        }
    }
}
