use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::{
    binary_to_text::{base32::B32Variant, BinaryToTextMode},
    Base32,
};
//use rfd::FileDialog;

impl ViewableCode for Base32 {}

impl View for Base32 {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        // ui.label("You can upload a file and encode its binary data in Base64. This website does not allow decoding of arbitrary inputs because it is impossible to know their contents before decoding, which is potentially dangerous.");
        // if ui.button("Select a File").clicked() {
        //     self.file = FileDialog::new().pick_file();
        // }

        ui.selectable_value(&mut self.variant, B32Variant::Rfc4648, "Standard");
        ui.selectable_value(&mut self.variant, B32Variant::ExtendedHex, "Extemded Hex");
        ui.selectable_value(&mut self.variant, B32Variant::WordSafe, "Word Safe");
        ui.add_space(10.0);
        match self.variant {
            B32Variant::Rfc4648 => ui.label("This variant defined as 'base32' by RFC 4684 is widely used and recognized. To avoid ambiguity in reading the characters 0, 1, and 8 are not included."),
            B32Variant::ExtendedHex => ui.label("This variant defined as 'base32hex' by RFC 4684 is less commonly used than the standard variant. It retains some ordering properties of the original data."),
            B32Variant::WordSafe => ui.label("The Word Safe variant of Base32 is used for geocaching. It is an example of a variant used to avoid forming words, which it accomplishes by not including any vowels."),

        };
        ui.add_space(10.0);
        ui.label("Encoding Mode");
        ui.selectable_value(&mut self.mode, BinaryToTextMode::Hex, "Hex")
            .on_hover_text("interpret input as hexcode");
        ui.selectable_value(&mut self.mode, BinaryToTextMode::Utf8, "Text")
            .on_hover_text("convert text to raw bytes");
        ui.add_space(10.0);
        ui.label("When padding is enabled the padding symbol `=` is added to the end until the length is a multiple of eight. Padding is ignored when decoding.");
        ui.checkbox(&mut self.use_padding, "Use Padding");
        ui.add_space(10.0);
        fill_code_columns(8, 4, ui, Box::new(self.chars_codes()));

        // if self.file.is_some() {
        //     let file_name = self.file.as_ref().unwrap().file_name().unwrap().to_str();
        //     ui.add_space(10.0);
        //     ui.label(format!("File: {}", file_name.unwrap()));
        //     ui.label("\nBase 32 Encoding\n");
        //     if ui.button("Copy to Clipboard").clicked() {
        //         ui.output_mut(|o| o.copied_text = self.encode_file().unwrap());
        //     }
        // }
    }
}
