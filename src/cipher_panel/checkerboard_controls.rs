use super::{View, ViewableCipher, _generic_components::*};
use crate::{ciphers::polybius::StraddlingCheckerboard, egui_aux::mono};
use eframe::egui::{DragValue, Ui};

impl ViewableCipher for StraddlingCheckerboard {}

impl View for StraddlingCheckerboard {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet).changed() {
            self.set_alphabet()
        }

        ui.add_space(16.0);
        let gap0 = 0..=(self.gaps.1 - 1);
        let gap1 = (self.gaps.0 + 1)..=9;

        ui.horizontal(|ui| {
            mono(ui, "First Gap ", None);
            ui.add(DragValue::new(&mut self.gaps.0).clamp_range(gap0));
        });

        ui.horizontal(|ui| {
            mono(ui, "Second Gap", None);
            ui.add(DragValue::new(&mut self.gaps.1).clamp_range(gap1));
        });

        ui.add_space(16.0);
        mono(ui, &self.cipher_page(), Some(15.0));
    }
}
