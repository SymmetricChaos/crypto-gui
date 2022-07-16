use super::{View, ViewableCipher, _generic_components::*};
use crate::{
    ciphers::polyalphabetic::{Quagmire, QuagmireVersion},
    egui_aux::mono,
};
use eframe::egui::Ui;

impl ViewableCipher for Quagmire {}

impl View for Quagmire {
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
            ui.selectable_value(&mut self.version, QuagmireVersion::V1, "V1");
            ui.selectable_value(&mut self.version, QuagmireVersion::V2, "V2");
            ui.selectable_value(&mut self.version, QuagmireVersion::V3, "V3");
            ui.selectable_value(&mut self.version, QuagmireVersion::V4, "V4");
        });

        ui.add_space(16.0);
        ui.label("Key Word");
        if control_string(ui, &mut self.ind_key_string).changed() {
            self.set_ind_key()
        }

        ui.add_space(16.0);
        ui.label("Key 1");
        if control_string(ui, &mut self.pt_key_string).changed() {
            self.set_pt_key()
        }

        if self.version == QuagmireVersion::V4 {
            ui.add_space(16.0);
            ui.label("Key 2");
            if control_string(ui, &mut self.ct_key_string).changed() {
                self.set_ct_key()
            }
        }

        ui.add_space(8.0);
        mono(ui, &self.show_pt_key(), None);
        ui.add_space(8.0);
        mono(ui, &self.show_ct_key(), None);
    }
}
