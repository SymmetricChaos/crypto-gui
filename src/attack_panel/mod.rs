use crate::{
    cipher_attacks::{caesar_attack::CaesarAttack, CipherAttack},
    ids::AttackId,
};
use egui::Ui;

pub mod caesar_attack_controls;

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

#[derive(Default)]
pub struct AttackInterface {
    // Text Standards
    caesar: CaesarAttack,
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
            AttackId::Substitution => todo!(),
            //_ => todo!("unable to get active code"),
        }
    }
}
