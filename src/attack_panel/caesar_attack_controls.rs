use eframe::egui::Ui;

use crate::cipher_attacks::caesar_attack::CaesarAttack;

use super::{text_score_group, View, ViewableAttack};

impl ViewableAttack for CaesarAttack {}

impl View for CaesarAttack {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        text_score_group(ui, &mut self.text_scorer);
    }
}
