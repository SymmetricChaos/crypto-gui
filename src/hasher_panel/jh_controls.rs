use hashers::jh::{Jh, JhHashLen};
use hashers::traits::StatefulHasher;
use utils::byte_formatting::ByteFormat;

use crate::ui_elements::UiElements;

use super::HasherFrame;

pub struct JhFrame {
    hash_len: JhHashLen,
    input_format: ByteFormat,
    output_format: ByteFormat,
}

impl Default for JhFrame {
    fn default() -> Self {
        Self {
            hash_len: JhHashLen::L256,
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl HasherFrame for JhFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/jh.rs",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.hash_len, JhHashLen::L224, "JH-224");
            ui.selectable_value(&mut self.hash_len, JhHashLen::L256, "JH-256");
            ui.selectable_value(&mut self.hash_len, JhHashLen::L384, "JH-384");
            ui.selectable_value(&mut self.hash_len, JhHashLen::L512, "JH-512");
        });
        todo!()
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;
        match self.hash_len {
            JhHashLen::L224 => Ok(self
                .output_format
                .byte_slice_to_text(Jh::init_224().update_and_finalize(&bytes))),
            JhHashLen::L256 => Ok(self
                .output_format
                .byte_slice_to_text(Jh::init_256().update_and_finalize(&bytes))),
            JhHashLen::L384 => Ok(self
                .output_format
                .byte_slice_to_text(Jh::init_384().update_and_finalize(&bytes))),
            JhHashLen::L512 => Ok(self
                .output_format
                .byte_slice_to_text(Jh::init_512().update_and_finalize(&bytes))),
        }
    }
}
