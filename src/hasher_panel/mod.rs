mod md4_controls;
mod md5_controls;
mod pearson_controls;
mod sha1_controls;
mod siphash_controls;

use egui::Ui;
use hashers::ids::{hasher_categories::HasherCategory, HasherId};

use self::{md4_controls::Md4Frame, md5_controls::Md5Frame, siphash_controls::SipHashFrame};

pub trait HasherFrame {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String);
    fn hash(&self, bytes: &[u8]) -> Vec<u8>;
    fn hash_to_string(&self, bytes: &[u8]) -> String;
}

// Quick simple combo box builder
fn combox_box(
    hasher: &[HasherId],
    active_hasher: &mut Option<HasherId>,
    hasher_category: HasherCategory,
    ui: &mut Ui,
) {
    ui.horizontal(|ui| {
        egui::ComboBox::from_id_source(hasher_category.to_string())
            .selected_text(hasher_category.to_string())
            .show_ui(ui, |ui| {
                for id in hasher {
                    ui.selectable_value(active_hasher, Some(*id), id.to_string());
                }
            });
        ui.menu_button("+", |ui| ui.label(hasher_category.description()))
    });

    ui.add_space(10.0);
}

#[derive(Default)]
pub struct HasherInterface {
    md4: Md4Frame,
    md5: Md5Frame,
    siphash: SipHashFrame,
}

impl HasherInterface {
    pub fn combo_boxes(&mut self, ui: &mut Ui, active_hasher: &mut Option<HasherId>) {
        combox_box(
            &[
                HasherId::Md4,
                HasherId::Md5,
                HasherId::Pearson,
                HasherId::SipHash,
            ],
            active_hasher,
            HasherCategory::Hasher,
            ui,
        );
    }

    pub fn get_active_hasher(&mut self, active_hasher: &HasherId) -> &mut dyn HasherFrame {
        match active_hasher {
            HasherId::Md4 => &mut self.md4,
            HasherId::Md5 => &mut self.md5,
            // HasherId::Pearson => &mut self.pearson,
            HasherId::SipHash => &mut self.siphash,
            _ => todo!("<<<RNG NOT FOUND>>>"),
        }
    }
}
