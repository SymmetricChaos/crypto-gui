use eframe::egui::Ui;
use egui::Slider;

use crate::{cipher_attacks::substitution_attack::SubstitutionAttack, egui_aux::subheading};

use super::{text_score_group, View, ViewableAttack};

impl ViewableAttack for SubstitutionAttack {}

impl View for SubstitutionAttack {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.label(subheading("Maximum Iterations"))
            .on_hover_text("Maximum number of times to try a new alphabet");
        ui.add(Slider::new(&mut self.num_trials, 50_000..=500_000));

        ui.label(subheading("Stopping Condition"));
        ui.label("After making this many attempts without improvement the attack will stop early. Higher values will search more for improvement while low values will run faster.");
        ui.add(Slider::new(&mut self.quit_number, 100..=10_000));

        text_score_group(ui, &mut self.text_scorer);

        ui.add_space(16.0);
    }
}