use egui::DragValue;
use hashers::ghash::Ghash;

use super::HasherFrame;
use crate::ui_elements::UiElements;

pub struct GhashFrame {
    hasher: Ghash,
}

impl Default for GhashFrame {
    fn default() -> Self {
        Self {
            hasher: Ghash::default(),
        }
    }
}

impl HasherFrame for GhashFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/ghash.rs",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );
        ui.add_space(4.0);

        ui.subheading("Additional Data Length");
        ui.label("Any number of bytes of the input can be specified as Additional Data. This is split off and processed separately with its own padding.");
        ui.add(DragValue::new(&mut self.hasher.ad_len));
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.subheading("H Value");
            ui.random_num_button(&mut self.hasher.h);
        });
        ui.label("H is the point at which the GHASH polynomial is evaluated.");
        ui.u128_hex_edit(&mut self.hasher.h);

        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.subheading("Constant Value");
            ui.random_num_button(&mut self.hasher.c);
        });
        ui.label("The constant value is the portion of the GHASH polynomial never multiplied by H. It is simply XORed into hash state before it is returned.");
        ui.u128_hex_edit(&mut self.hasher.c);

        ui.add_space(16.0);
    }

    crate::hash_string! {}
}
