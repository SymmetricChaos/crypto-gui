use super::{View, ViewableCipher, _generic_components::*};
use crate::{
    ciphers::polyalphabetic::{Hutton, HuttonVersion},
    egui_aux::mono,
};
use eframe::egui::Ui;

impl ViewableCipher for Hutton {}

impl View for Hutton {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.set_alphabet()
        }

        ui.add_space(16.0);
        ui.label("Select Version");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.version, HuttonVersion::V1, "V1");
            ui.selectable_value(&mut self.version, HuttonVersion::V2, "V2");
        });

        ui.add_space(16.0);
        ui.label("Password");
        if control_string(ui, &mut self.password_string).changed() {
            self.set_password()
        }

        ui.add_space(16.0);
        ui.label("Key Word");
        if control_string(ui, &mut self.key_string).changed() {
            self.set_key()
        }

        ui.add_space(8.0);
        mono(ui, &self.keyed_alphabet(), None);
    }
}
