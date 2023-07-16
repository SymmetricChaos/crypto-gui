use super::CodeFrame;
use crate::ui_elements::UiElements;
use codes::binary_to_text::base64::{B64Variant, Base64};

pub struct Base64Frame {
    code: Base64,
}

impl Default for Base64Frame {
    fn default() -> Self {
        Self {
            code: Default::default(),
        }
    }
}

impl CodeFrame for Base64Frame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.group(|ui| {
            ui.subheading("Variant");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.code.variant, B64Variant::Standard, "Standard");
                ui.selectable_value(
                    &mut self.code.variant,
                    B64Variant::UrlSafe,
                    "URL and Filename Safe",
                );
            });
        });

        ui.add_space(16.0);
        match self.code.variant {
            B64Variant::Standard => {
                ui.label("The most commonly used Base64 variant is defined by RFC 4684 section 4.")
            }
            B64Variant::UrlSafe => ui.label("URL and Filename Safe variant is defioned in RFC 4684 section 5 to be used in situations where the + and / characters might have special use defined for them. They are replaced by - and _."),
        };
        ui.add_space(16.0);
        ui.binary_to_text_input_mode(&mut self.code.mode);
        ui.add_space(16.0);
        ui.label("When padding is enabled the padding symbol `=` is added to the end until the length is a multiple of three. Padding is ignored when decoding.");
        ui.checkbox(&mut self.code.use_padding, "Use Padding");
        ui.add_space(16.0);
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
        ui.fill_code_columns(16, 4, Box::new(self.code.chars_codes()));
    }

    fn code(&self) -> &dyn codes::traits::Code {
        &self.code
    }
}
