use crate::ui_elements::UiElements;

use super::CodeFrame;
use codes::compression::{run_length::RunLengthEncoding, run_length_bytes::RunLengthEncodingBytes};

pub struct RleFrame {
    byte_code: RunLengthEncodingBytes,
}

impl Default for RleFrame {
    fn default() -> Self {
        Self {
            byte_code: Default::default(),
        }
    }
}

impl CodeFrame for RleFrame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/compression/run_length_bytes.rs",
        );


        // {
        //     let this = &mut *ui;
        //     let mut changed = false;
        //     egui::CollapsingHeader::new("Input Format")
        //         .default_open(true)
        //         .show(this, |ui| {
        //             ui.label(
        //                 "Input can be text, hexadecimal, Base64, or binary. All interpreted as bytes.",
        //             );
        //             ui.horizontal(|ui| {
        //                 for variant in ByteFormat::iter() {
        //                     if ui
        //                         .selectable_value(&mut self.byte_code., variant, variant.to_string())
        //                         .clicked()
        //                     {
        //                         changed = true;
        //                     }
        //                 }
        //             });
        //         });

        //     this.add_space(8.0);

        //     egui::CollapsingHeader::new("Output Format")
        //         .default_open(true)
        //         .show(this, |ui| {
        //             ui.label(
        //                 "Output can be text, hexadecimal, Base64, or binary. All interpreted as bytes.",
        //             );
        //             ui.horizontal(|ui| {
        //                 for variant in ByteFormat::iter() {
        //                     if ui
        //                         .selectable_value(output, variant, variant.to_string())
        //                         .clicked()
        //                     {
        //                         changed = true;
        //                     }
        //                 }
        //             });
        //         });
        //     changed
        // };

        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.text_code
    }
}
