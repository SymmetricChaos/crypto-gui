use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::{
    binary_to_text::{base64::B64Variant, BinaryToTextMode},
    Base64,
};
//use rfd::FileDialog;

impl ViewableCode for Base64 {}

impl View for Base64 {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        // ui.label("You can upload a file and encode its binary data in Base64. This website does not allow decoding of arbitrary inputs because it is impossible to know their contents before decoding, which is potentially dangerous.");
        // if ui.button("Select a File").clicked() {
        //     self.file = FileDialog::new().pick_file();
        // }
        ui.selectable_value(&mut self.variant, B64Variant::Standard, "Standard");
        ui.selectable_value(
            &mut self.variant,
            B64Variant::UrlSafe,
            "URL and Filename Safe",
        );
        ui.add_space(10.0);
        match self.variant {
            B64Variant::Standard => {
                ui.label("The most commonly used Base64 variant is defined by RFC 4684 section 4.")
            }
            B64Variant::UrlSafe => ui.label("URL and Filename Safe variant is defioned in RFC 4684 section 5 to be used in situations where the + and / characters might have special use defined for them. They are replaced by - and _."),
        };
        ui.add_space(10.0);
        ui.label("Encoding Mode");
        ui.selectable_value(&mut self.mode, BinaryToTextMode::Hex, "Hex")
            .on_hover_text("interpret input as hexcode");
        ui.selectable_value(&mut self.mode, BinaryToTextMode::Utf8, "Text")
            .on_hover_text("convert text to raw bytes");
        ui.add_space(10.0);
        ui.label("When padding is enabled the padding symbol `=` is added to the end until the length is a multiple of three. Padding is ignored when decoding.");
        ui.checkbox(&mut self.use_padding, "Use Padding");
        ui.add_space(10.0);
        fill_code_columns(16, 4, ui, Box::new(self.chars_codes()));
        ui.add_space(10.0);
        // if self.file.is_some() {
        //     let file_name = self.file.as_ref().unwrap().file_name().unwrap().to_str();
        //     ui.add_space(10.0);
        //     ui.label(format!("File: {}", file_name.unwrap()));
        //     ui.label("\nBase 64 Encoding\n");
        //     if ui.button("Copy to Clipboard").clicked() {
        //         ui.output_mut(|o| o.copied_text = self.encode_file().unwrap());
        //     }
        // }
    }
}
