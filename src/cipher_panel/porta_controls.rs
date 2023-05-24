use ciphers::{polyalphabetic::Porta, Cipher};
use egui::Ui;

use crate::egui_aux::mono;

use super::{CipherFrame, _generic_components::control_string};

#[derive(Default)]
pub struct PortaFrame {
    cipher: Porta,
    key_word_string: String,
}

impl CipherFrame for PortaFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        // randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Keyword");
        if control_string(ui, &mut self.key_word_string).changed() {
            self.cipher.assign_key(&self.key_word_string)
        }
        ui.add_space(16.0);

        ui.label("Tableaux");
        for row in self.cipher.tableaux() {
            ui.label(mono(row));
        }
        ui.add_space(16.0);

        // This is possible but not yet implemented
        // ui.label("Mode");
        // ui.horizontal(|ui| {
        //     ui.selectable_value(&mut self.mode, CylicKey, "Cyclic");
        //     ui.selectable_value(&mut self.mode, Autokey, "Autokey");
        //     ui.selectable_value(&mut self.mode, ProgKey, "Progressive");
        // });

        // if self.mode == ProgKey {
        //     ui.add_space(16.0);
        //     ui.label("Step size");
        //     let alpha_range = 0..=(self.alphabet_len() - 1);
        //     ui.add(Slider::new(&mut self.prog_shift, alpha_range));
        //     ui.add_space(16.0);
        // }

        // match self.multikey {
        //     true => {
        //         ui.horizontal(|ui| {
        //             ui.label("Key Words");
        //             ui.checkbox(&mut self.multikey, "Multikey");
        //         });
        //         ui.add(TextEdit::singleline(&mut self.key_words[0]).font(TextStyle::Monospace));
        //         ui.add(TextEdit::singleline(&mut self.key_words[1]).font(TextStyle::Monospace));
        //         ui.add(TextEdit::singleline(&mut self.key_words[2]).font(TextStyle::Monospace));
        //         ui.add(TextEdit::singleline(&mut self.key_words[3]).font(TextStyle::Monospace));
        //         ui.add(TextEdit::singleline(&mut self.key_words[4]).font(TextStyle::Monospace));
        //     }
        //     false => {
        //         ui.horizontal(|ui| {
        //             ui.label("Key Word ");
        //             ui.checkbox(&mut self.multikey, "Multikey");
        //         });
        //         ui.add(TextEdit::singleline(&mut self.key_words[0]).font(TextStyle::Monospace));
        //     }
        // }
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {
        *self = Self::default()
    }
}
