use egui::{FontId, RichText};
use hashers::{
    tiger::{Tiger, TigerVersion},
    tiger_arrays::*,
};

use crate::ui_elements::UiElements;

use super::HasherFrame;

#[derive(Default)]
pub struct TigerFrame {
    hasher: Tiger,
}

impl HasherFrame for TigerFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.subheading("Version");
        ui.label("In V1 the first padding byte is 0x01 and in V2 the first padding byte is 0x80.");
        ui.selectable_value(&mut self.hasher.version, TigerVersion::One, "V1");
        ui.selectable_value(&mut self.hasher.version, TigerVersion::Two, "V2");

        ui.collapsing("Tiger SBOXes (very large)", |ui| {
            for (i, sbox) in [T1, T2, T3, T4].iter().enumerate() {
                ui.label(format!("T{}:", i + 1));
                egui::Grid::new(format!("tiger_array{i}"))
                    .num_columns(4)
                    .striped(true)
                    .show(ui, |ui| {
                        for (n, b) in sbox.iter().enumerate() {
                            if n % 4 == 0 && n != 0 {
                                ui.end_row()
                            }
                            ui.label(
                                RichText::from(format!("{:016X}", b)).font(FontId::monospace(14.0)),
                            );
                        }
                    });
                ui.add_space(8.0);
            }
        });
    }

    crate::hash_string! {}
}
