use crate::{
    cipher_attacks::{
        caesar_attack::CaesarAttack, substitution_attack::SubstitutionAttack, CipherAttack,
        TextScorer,
    },
    egui_aux::subheading,
    ids::AttackId,
};
use egui::Ui;

pub mod caesar_attack_controls;
pub mod substitution_attack_controls;

pub trait ViewableAttack: View + CipherAttack {}

pub trait View {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String);
}

// Quick simple combo box builder
fn combox_box(
    code: &[AttackId],
    identifier: &'static str,
    active_attack: &mut AttackId,
    ui: &mut Ui,
) {
    egui::ComboBox::from_id_source(identifier)
        .selected_text(identifier)
        .show_ui(ui, |ui| {
            for id in code {
                ui.selectable_value(active_attack, *id, id.to_string());
            }
        });
    ui.add_space(10.0);
}

fn text_score_group(ui: &mut Ui, scorer: &mut TextScorer) {
    ui.group(|ui| {
        ui.label(subheading("Text Scoring"));
        ui.horizontal(|ui| {
            ui.selectable_value(scorer, TextScorer::Bigram, "2-Gram");
            ui.selectable_value(scorer, TextScorer::Trigram, "3-Gram");
            ui.selectable_value(scorer, TextScorer::Quadgram, "4-Gram");
        });
    });
}

#[derive(Default)]
pub struct AttackInterface {
    // Substitution
    caesar: CaesarAttack,
    substitution: SubstitutionAttack,
}

impl AttackInterface {
    pub fn combo_boxes(&mut self, ui: &mut Ui, active_attack: &mut AttackId) {
        combox_box(
            &[AttackId::Caesar, AttackId::Substitution],
            "Substitution",
            active_attack,
            ui,
        );
    }

    pub fn get_active_attack(&mut self, active_attack: &AttackId) -> &mut dyn ViewableAttack {
        match active_attack {
            AttackId::Caesar => &mut self.caesar,
            AttackId::Substitution => &mut self.substitution,
            //_ => todo!("unable to get active code"),
        }
    }
}
