use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::compression::run_length_bytes::{RleMethod, RunLengthEncodingBytes};

pub struct RleFrame {
    code: RunLengthEncodingBytes,
}

impl Default for RleFrame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for RleFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/compression/run_length_bytes.rs",
        );

        ui.group(|ui| {
            ui.subheading("Mode");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.method, RleMethod::OneByte, "One Byte");
                ui.selectable_value(&mut self.code.method, RleMethod::Leb128, "LEB128");
            });
        });

        ui.byte_io_mode_cipher(&mut self.code.input_format, &mut self.code.output_format);

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
