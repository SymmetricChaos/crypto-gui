use eframe::egui::Ui;

use crate::{
    cipher_attacks::caesar_attack::CaesarAttack, cipher_panel::_generic_components::control_string,
    egui_aux::subheading,
};

use super::{text_score_group, View, ViewableAttack};

impl ViewableAttack for CaesarAttack {}

impl View for CaesarAttack {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.label(subheading("Plaintext Alphabet"));
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.set_alphabet()
        }

        text_score_group(ui, &mut self.text_scorer);

        ui.add_space(16.0);
    }
}
