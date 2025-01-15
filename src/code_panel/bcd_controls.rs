use crate::ui_elements::UiElements;

use super::CodeFrame;
use codes::mathematical::binary_coded_decimal::{BinaryCodedDecimal, WordWidth};
use utils::byte_formatting::ByteFormat;

pub struct BcdFrame {
    code: BinaryCodedDecimal,
}

impl Default for BcdFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for BcdFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/binary_coded_decimal.rs",
        );

        ui.add_space(8.0);
        ui.group(|ui| {
            ui.subheading("Width");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.width, WordWidth::W32, "32-bit Word");
                ui.selectable_value(&mut self.code.width, WordWidth::W64, "64-bit Word");
                ui.selectable_value(&mut self.code.width, WordWidth::W128, "128-bit Word");
            });
        });

        ui.add_space(8.0);
        ui.label("If the sign tetrade is excluded BCD and store one additional digit.");
        ui.checkbox(&mut self.code.signed, "Signed");

        ui.add_space(12.0);
        ui.group(|ui| {
            ui.subheading("Byte Format");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.formatting, ByteFormat::Hex, "Hexadecimal");
                ui.selectable_value(&mut self.code.formatting, ByteFormat::Base64, "Base64");
                ui.selectable_value(&mut self.code.formatting, ByteFormat::Binary, "Binary");
            });
        });
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
