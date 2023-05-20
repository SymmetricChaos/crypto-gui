use codes::binary_to_text::BinaryToTextMode;
use eframe::egui;
use std::fmt::Display;

use crate::egui_aux::mono_strong;

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
// use std::path::PathBuf;

// use rfd::FileDialog;
// #[cfg(not(target_arch = "wasm32"))]
// pub fn upload_file(ui: &mut egui::Ui, file: &mut Option<PathBuf>) {
//     if ui.button("Upload File").clicked() {
//         *file = FileDialog::new().pick_file();
//     }
//     let file_name = file.as_ref().unwrap().file_name().unwrap().to_str();
//     ui.add_space(10.0);
//     ui.label(format!("{}", file_name.unwrap()));
// }
// #[cfg(target_arch = "wasm32")]
// pub fn upload_file(ui: &mut egui::Ui, file: &mut Option<PathBuf>) {}

// #[cfg(not(target_arch = "wasm32"))]
// pub fn encode_file_and_save(
//     ui: &mut egui::Ui,
//     code: &dyn BinaryToText,
//     source_file: Option<PathBuf>,
// ) {
//     if ui.button("Download Encoded File").clicked() {
//         let target_file = FileDialog::new().add_filter("", &[".txt"]).save_file();
//         if let Some(file) = target_file {
//             std::fs::write(file, code.encode_file(source_file).unwrap()).unwrap()
//         }
//     }
// }

// #[cfg(target_arch = "wasm32")]
// pub fn encode_file_and_save(file: Option<PathBuf>) {}
