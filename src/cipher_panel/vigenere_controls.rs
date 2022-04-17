use eframe::egui::Ui;
use rand::prelude::StdRng;
use super::{generic_components::*, View};
use crate::ciphers::{PolyMode::*, Vigenere};
use eframe::egui::{Slider, TextEdit, TextStyle};

impl View for Vigenere {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng, _errors: &mut String) {
        randomize_reset(ui, self, rng);
        ui.add_space(16.0);

        input_alphabet(ui, self.control_alphabet());
        ui.add_space(16.0);

        ui.label("Mode");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mode, CylicKey, "Cyclic");
            ui.selectable_value(&mut self.mode, Autokey, "Autokey");
            ui.selectable_value(&mut self.mode, ProgKey, "Progressive");
        });

        if self.mode == ProgKey {
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
