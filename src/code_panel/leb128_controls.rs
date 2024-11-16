use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::{mathematical::leb128::Leb128, traits::Code};
use utils::byte_formatting::ByteFormat;

pub struct Leb128Frame {
    code: Leb128,
    // words_string: String,
}

impl Default for Leb128Frame {
    fn default() -> Self {
        Self {
            code: Default::default(),
            // words_string: String::new(),
        }
    }
}

impl CodeFrame for Leb128Frame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/mathematical/leb128.rs",
        );

        ui.add_space(8.0);
        ui.checkbox(&mut self.code.signed, "Signed (Include Negative Integers)");
        ui.add_space(8.0);

        ui.checkbox(&mut self.code.spaced, "Encode Each Group Seperately");
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.subheading("Byte Format");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.byte_format, ByteFormat::Hex, "Hex");
                ui.selectable_value(&mut self.code.byte_format, ByteFormat::Base64, "Base64");
                ui.selectable_value(&mut self.code.byte_format, ByteFormat::Binary, "Binary");
            });
        });
        ui.add_space(8.0);

        if self.code.signed {
            ui.heading("Signed Examples");
            ui.monospace(
                "0, -12, 345, -6789, 10111, -213141, 5161718, -19202122, 232425262, -7282930313",
            );
            ui.monospace(
                "00, 74, d902, fb4a, ffce00, ebfe72, f685bb02, b6ffeb76, ae8eeaee00, f79a9def64",
            );
        } else {
            ui.heading("Unsigned Examples");
            ui.monospace(
                "0, 12, 345, 6789, 10111, 213141, 5161718, 19202122, 232425262, 7282930313",
            );
            ui.monospace(
                "00, 0c, d902, 8535, ff4e, 95810d, f685bb02, ca809409, ae8eea6e, 89e5e2901b",
            );
        }

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn Code {
        &self.code
    }
}
