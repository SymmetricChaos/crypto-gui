use egui::{FontId, RichText};
use hashers::{
    auxiliary::tiger_arrays::*,
    tiger::{Tiger, TigerVersion},
};

use crate::ui_elements::UiElements;

use super::HasherFrame;

#[derive(Default)]
pub struct TigerFrame {
    hasher: Tiger,
}

impl HasherFrame for TigerFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/tiger.rs",
        );

        ui.subheading("Version");
        ui.label("In V1 the first padding byte is 0x01 and in V2 the first padding byte is 0x80. There is no other difference.");
        ui.selectable_value(&mut self.hasher.version, TigerVersion::One, "V1");
        ui.selectable_value(&mut self.hasher.version, TigerVersion::Two, "V2");
        ui.add_space(16.0);

        ui.subheading("Tiger S-boxes (very large)");
        for (i, sbox) in [T1, T2, T3, T4].iter().enumerate() {
            ui.collapsing(format!("T{}", i + 1), |ui| {
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
            });
            ui.add_space(8.0);
        }
    }

    crate::hash_string! {}
}
