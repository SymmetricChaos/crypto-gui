use super::{
    generic_components::{binary_to_text_input_mode, fill_code_columns},
    View, ViewableCode,
};
use crate::codes::binary_to_text::ascii85::{Ascii85, Ascii85Variant};
use rfd::FileDialog;

impl ViewableCode for Ascii85 {}

impl View for Ascii85 {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        ui.selectable_value(&mut self.variant, Ascii85Variant::Btoa, "btoa");
        ui.selectable_value(&mut self.variant, Ascii85Variant::Adobe, "Adobe");
        ui.selectable_value(&mut self.variant, Ascii85Variant::Ipv6, "IPv6");
        ui.selectable_value(&mut self.variant, Ascii85Variant::ZeroQm, "Z85 (ZeroQM)");
        ui.add_space(10.0);
        match self.variant {
            Ascii85Variant::Btoa => ui.label("The original Ascii85 encoding created for the btoa (binary-to-ASCII) utility simply uses the printing ASCII characters from '!' to 'u' in order. For efficiency with real world inputs it has two special encodings: 0x00000000 (the all zero word) is encoded as just 'z' while 0x20202020 (the sequence of four ASCII spaces) is encoded as just 'y'."),
            Ascii85Variant::Ipv6 => ui.label("While it can be used for arbitary data this variant was created to allow encoding for IPv6 addresses. For this use the 85 character set is the most efficient possible. The selection of characters is different in order to avoid conflicts."),
            Ascii85Variant::ZeroQm => ui.label("Z85 variant deliberately excludes the the ASCII quote characters to make the encoding safe to used in quoted strings and thus much easier to include in source code. While no special encoding is used for the all zero word (0x00000000) the placement of zero as the first digit means that runs of zero bytes appear as runs of zeroes in encoded text."),
            Ascii85Variant::Adobe => ui.label("The Adobe variant of is used in PDFs and differs from the original btoa only in that it does not have the 'y' special rule and that Adobe marks the start and end of the encoded string differently."),
        };
        ui.add_space(10.0);
        binary_to_text_input_mode(ui, &mut self.mode);
        ui.add_space(10.0);
        ui.label("You can upload a file and encode its binary data as text. Decoding files is not supported as it is impossible to know the contents.");
        if ui.button("Upload File").clicked() {
            self.file = FileDialog::new().pick_file();
        }
        if self.file.is_some() {
            let file_name = self.file.as_ref().unwrap().file_name().unwrap().to_str();
            ui.add_space(10.0);
            ui.label(format!("{}", file_name.unwrap()));
            if ui.button("Download Encoded File").clicked() {
                let target_file = FileDialog::new().add_filter("", &[".txt"]).save_file();
                if let Some(file) = target_file {
                    std::fs::write(file, self.encode_file().unwrap()).unwrap()
                }
            }
        }
        ui.add_space(32.0);
        fill_code_columns(17, 5, ui, Box::new(self.chars_codes()));
    }
}
