use super::{generic_components::fill_code_columns, View, ViewableCode};
use crate::codes::Base64;
//use rfd::FileDialog;

impl ViewableCode for Base64 {}

impl View for Base64 {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _errors: &mut String) {
        // ui.label("You can upload a file and encode its binary data in Base64. This website does not allow decoding of arbitrary inputs because it is impossible to know their contents before decoding, which is potentially dangerous.");
        // if ui.button("Select a File").clicked() {
        //     self.file = FileDialog::new().pick_file();
        // }

        fill_code_columns(16, 4, ui, Box::new(self.chars_codes()));

        if self.file.is_some() {
            let file_name = self.file.as_ref().unwrap().file_name().unwrap().to_str();
            ui.add_space(10.0);
            ui.label(format!("File: {}", file_name.unwrap()));
            ui.label("\nBase 64 Encoding\n");
            if ui.button("Copy to Clipboard").clicked() {
                ui.output_mut(|o| o.copied_text = self.encode_file().unwrap());
            }
        }
    }
}
