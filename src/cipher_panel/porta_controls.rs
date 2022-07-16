use super::{View, ViewableCipher, _generic_components::*};
use crate::{ciphers::polyalphabetic::Porta, egui_aux::mono};
use eframe::egui::Ui;

impl ViewableCipher for Porta {}

impl View for Porta {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Keyword");
        if control_string(ui, &mut self.key).changed() {
            self.set_key()
        }
        ui.add_space(16.0);

        ui.label("Tableaux");
        for row in self.tableaux() {
            mono(ui, row, None);
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
}
