use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::binary_to_text::base32::{B32Variant, Base32};

pub struct Base32Frame {
    code: Base32,
}

impl Default for Base32Frame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for Base32Frame {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/codes/src/binary_to_text/base32.rs",
        );
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.subheading("Variant");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.variant, B32Variant::Rfc4648, "Standard");
                ui.selectable_value(
                    &mut self.code.variant,
                    B32Variant::ExtendedHex,
                    "Extended Hex",
                );
                ui.selectable_value(&mut self.code.variant, B32Variant::WordSafe, "Word Safe");
            });
        });

        ui.add_space(8.0);
        match self.code.variant {
            B32Variant::Rfc4648 => ui.label("This variant defined as 'base32' by RFC 4684 is widely used and recognized. To avoid ambiguity in reading the characters 0, 1, and 8 are not included."),
            B32Variant::ExtendedHex => ui.label("This variant defined as 'base32hex' by RFC 4684 is less commonly used than the standard variant. It retains some ordering properties of the original data."),
            B32Variant::WordSafe => ui.label("This word safe variant of Base32 is used for geocaching and is used to avoid forming words, which it accomplishes by not including any vowels."),
        };
        ui.add_space(8.0);
        ui.binary_to_text_input_mode(&mut self.code.mode);
        ui.add_space(8.0);
        ui.subheading("Padding");
        ui.label("When padding is enabled the padding symbol `=` is added to the end until the length is a multiple of eight. Padding is ignored when decoding.");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.code.use_padding, true, "On");
            ui.selectable_value(&mut self.code.use_padding, false, "Off");
        });

        ui.add_space(8.0);
        // use rfd::FileDialog;
        // ui.label("You can upload a file and encode its binary data as text. Decoding files is not supported as it is impossible to know the contents.");
        // if ui.button("Upload File").clicked() {
        //     self.file = FileDialog::new().pick_file();
        // }
        // if self.file.is_some() {
        //     let file_name = self.file.as_ref().unwrap().file_name().unwrap().to_str();
        //     ui.add_space(10.0);
        //     ui.label(format!("{}", file_name.unwrap()));
        //     if ui.button("Download Encoded File").clicked() {
        //         let target_file = FileDialog::new().add_filter("", &[".txt"]).save_file();
        //         if let Some(file) = target_file {
        //             std::fs::write(file, self.encode_file().unwrap()).unwrap()
        //         }
        //     }
        // }
        // ui.add_space(32.0);
        ui.fill_code_columns(8, 4, Box::new(self.code.chars_codes()));
        ui.add_space(16.0);
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
