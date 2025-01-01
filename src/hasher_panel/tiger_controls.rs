use egui::{FontId, RichText};
use hashers::{
    auxiliary::tiger_arrays::*,
    tiger::{Tiger, TigerVersion},
    traits::StatefulHasher,
};
use utils::byte_formatting::ByteFormat;

use crate::ui_elements::UiElements;

use super::HasherFrame;

pub struct TigerFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    version: TigerVersion,
}

impl Default for TigerFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            version: TigerVersion::One,
        }
    }
}

impl HasherFrame for TigerFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/tiger.rs",
        );

        ui.subheading("Version");
        ui.label("In V1 the first padding byte is 0x01 and in V2 the first padding byte is 0x80. There is no other difference.");
        ui.selectable_value(&mut self.version, TigerVersion::One, "V1");
        ui.selectable_value(&mut self.version, TigerVersion::Two, "V2");
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

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        Ok(self
            .output_format
            .byte_slice_to_text(&Tiger::init(self.version).hash(&bytes)))
    }
}
