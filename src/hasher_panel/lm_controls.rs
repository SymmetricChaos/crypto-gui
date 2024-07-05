use crate::ui_elements::UiElements;

use super::HasherFrame;
use hashers::{
    errors::HasherError,
    lm::{des::Des, des_functions::expand_56_to_64, lm::LM_WORD, Lm},
    traits::ClassicHasher,
};
use utils::{
    byte_formatting::ByteFormat, preset_alphabet::Alphabet, text_functions::filter_string,
};

pub struct LmFrame {
    hasher: Lm,
    example_pass: String,
    example_pass_processed: String,
    h1: String,
    h2: String,
    des: Des,
}

impl Default for LmFrame {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
            example_pass: String::from("PassWord"),
            example_pass_processed: String::from("PASSWORD\0\0\0\0\0\0"),
            h1: String::from("e52cac67419a9a22"),
            h2: String::from("4a3b108f3fa6cb6d"),
            des: Des::default(),
        }
    }
}

impl LmFrame {}

impl HasherFrame for LmFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.add_space(16.0);

        ui.collapsing("Output Format", |ui| {
            ui.label("Output can be hexadecimal representing bytes or Base64 representing bytes.");
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.hasher.output_format,
                    ByteFormat::Hex,
                    "Hexadecimal",
                );
                ui.selectable_value(&mut self.hasher.output_format, ByteFormat::Base64, "Base64");
            });
        });

        ui.add_space(16.0);

        ui.subheading("Example");
        if ui.control_string(&mut self.example_pass).changed() {
            filter_string(&mut self.example_pass, &Alphabet::Ascii94.slice());
            self.example_pass.truncate(14);
            self.example_pass_processed = self.example_pass.to_ascii_uppercase().to_string();
            while self.example_pass_processed.len() < 14 {
                self.example_pass_processed.push('\0')
            }

            let k1 = expand_56_to_64(
                self.example_pass_processed.as_bytes()[0..7]
                    .try_into()
                    .unwrap(),
            );
            let k2 = expand_56_to_64(
                self.example_pass_processed.as_bytes()[7..14]
                    .try_into()
                    .unwrap(),
            );

            self.des.ksa(k1).unwrap();
            self.h1 = self
                .hasher
                .output_format
                .byte_slice_to_text(&self.des.encrypt_block(LM_WORD).to_be_bytes());
            self.des.ksa(k2).unwrap();
            self.h2 = self
                .hasher
                .output_format
                .byte_slice_to_text(&self.des.encrypt_block(LM_WORD).to_be_bytes());
        }

        ui.add_space(8.0);
        ui.subheading("Capitalized");
        ui.mono_strong(format!("{}", self.example_pass_processed));

        ui.add_space(8.0);
        ui.subheading("Split");
        ui.label("Note that if the password is less than 8 characters in length the second half is always empty and is always \"hashed\" to the same value");
        ui.add_space(2.0);
        ui.mono_strong(format!(
            "{}   {}",
            &self.example_pass_processed[0..7],
            &self.example_pass_processed[7..14]
        ));

        ui.add_space(8.0);
        ui.subheading("Hashed (Encrypted)");
        ui.label("The two halves are both used as DES keys to encrypt the string KGS!@#$% and the results concatenated as a hash.");
        ui.add_space(2.0);
        ui.mono_strong(format!("{}   {}", &self.h1, &self.h2));

        ui.add_space(16.0);
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        self.hasher.hash_bytes_from_string(text)
    }
}
