use super::HasherFrame;
use crate::ui_elements::UiElements;
use hashers::ripemd::ripemd::{RipeMd, RipeMdVariant};
use strum::IntoEnumIterator;

pub struct RipeMdFrame {
    hasher: RipeMd,
}

impl Default for RipeMdFrame {
    fn default() -> Self {
        Self {
            hasher: RipeMd::default(),
        }
    }
}

impl RipeMdFrame {}

impl HasherFrame for RipeMdFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/ripemd",
        );

        ui.byte_io_mode_hasher(
            &mut self.hasher.input_format,
            &mut self.hasher.output_format,
        );
        ui.add_space(16.0);

        ui.subheading("RIPEMD Variants");
        ui.horizontal(|ui| {
            for variant in RipeMdVariant::iter() {
                ui.selectable_value(&mut self.hasher.variant, variant, variant.to_string());
            }
        });
    }

    crate::hash_string! {}
}
