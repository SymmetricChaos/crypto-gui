use eframe::egui::Ui;

use crate::{
    cipher_attacks::{caesar_attack::CaesarAttack, TextScore},
    cipher_panel::_generic_components::control_string,
    egui_aux::subheading,
};

use super::{View, ViewableAttack};

impl ViewableAttack for CaesarAttack {}

impl View for CaesarAttack {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.label(subheading("Alphabet"));
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.set_alphabet()
        }

        ui.group(|ui| {
            ui.label(subheading("Text Scoring"));
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.text_scorer, TextScore::Bigram, "2-Gram");
                ui.selectable_value(&mut self.text_scorer, TextScore::Trigram, "3-Gram");
                ui.selectable_value(&mut self.text_scorer, TextScore::Quadgram, "4-Gram");
            });
        });

        ui.add_space(16.0);
    }
}
