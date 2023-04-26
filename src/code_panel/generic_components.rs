use std::fmt::Display;

use crate::codes::binary_to_text::BinaryToTextMode;
use crate::egui_aux::mono_strong;
use eframe::egui::{self};

pub fn fill_code_columns<T: Display, S: Display>(
    nrows: usize,
    ncols: usize,
    ui: &mut egui::Ui,
    iter: Box<dyn Iterator<Item = (T, S)> + '_>,
) {
    ui.columns(ncols, |columns| {
        let mut ctr = 0;
        let mut col = 0;
        for (c, code) in iter {
            let pair = format!("{}  {} ", c, code);
            mono_strong(&mut columns[col], &pair, Some(18.0));
            ctr += 1;
            if ctr % nrows == 0 {
                col += 1
            }
        }
    });
}

pub fn code_button_columns(
    nrows: usize,
    ncols: usize,
    ui: &mut egui::Ui,
    target: &mut String,
    space: &str,
    iter: Box<dyn Iterator<Item = (char, &str)> + '_>,
) {
    ui.columns(ncols, |columns| {
        let mut ctr = 0;
        let mut col = 0;
        for (c, code) in iter {
            let pair = format!("{}  {} ", c, code);
            if columns[col].button(&pair).clicked() {
                if !target.is_empty() {
                    target.push_str(space);
                }
                target.push_str(code)
            }
            ctr += 1;
            if ctr % nrows == 0 {
                col += 1
            }
        }
    });
}

pub fn binary_to_text_input_mode(ui: &mut egui::Ui, current_value: &mut BinaryToTextMode) {
    ui.label("Encoding Mode");
    ui.selectable_value(current_value, BinaryToTextMode::Hex, "Hex")
        .on_hover_text("interpret input as hexcode");
    ui.selectable_value(current_value, BinaryToTextMode::Utf8, "Text")
        .on_hover_text("convert text to raw bytes");
}

// pub fn upload_and_save_file(file: Option<PathBuf>) {
//     use rfd::FileDialog;
//     ui.label("You can upload a file and encode its binary data as text. Decoding files is not supported as it is impossible to know the contents.");
//     if ui.button("Upload File").clicked() {
//         file = FileDialog::new().pick_file();
//     }
//     if self.file.is_some() {
//         let file_name = file.as_ref().unwrap().file_name().unwrap().to_str();
//         ui.add_space(10.0);
//         ui.label(format!("{}", file_name.unwrap()));
//         if ui.button("Download Encoded File").clicked() {
//             let target_file = FileDialog::new().add_filter("", &[".txt"]).save_file();
//             if let Some(file) = target_file {
//                 std::fs::write(file, self.encode_file().unwrap()).unwrap()
//             }
//         }
//     }
//     ui.add_space(32.0);
//     fill_code_columns(17, 5, ui, Box::new(self.chars_codes()));
// }
