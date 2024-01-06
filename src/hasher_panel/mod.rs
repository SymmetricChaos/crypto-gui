use egui::Ui;

pub trait ClassicHasherFrame {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String);
    fn hasher(&self) -> &dyn ClassicHasher;
}

// Quick simple combo box builder
fn combox_box(
    hasher: &[HasherId],
    active_hasher: &mut Option<RngId>,
    hasher_category: RngCategory,
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
pub struct HasherInterface {}

impl HasherInterface {
    pub fn combo_boxes(&mut self, ui: &mut Ui, active_hasher: &mut Option<RngId>) {
        combox_box(
            &[HasherId::MD4, HasherId::MD5],
            active_hasher,
            HasherCategory::Classic,
            ui,
        );
    }

    pub fn get_active_hasher(&mut self, active_hasher: &HasherId) -> &mut dyn ClassicRngFrame {
        match active_hasher {
            _ => todo!("<<<RNG NOT FOUND>>>"),
        }
    }
}
