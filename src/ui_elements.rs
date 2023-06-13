use std::fmt::Display;

use codes::binary_to_text::BinaryToTextMode;
use eframe::egui::RichText;
use egui::{Color32, Label, TextStyle, Ui};
use utils::grid::{str_to_char_grid, Grid};

use crate::cipher_panel::CipherFrame;

pub fn subheading<T: ToString>(text: T) -> RichText {
    RichText::from(text.to_string()).size(16.0)
}

pub fn mono<T: ToString>(text: T) -> RichText {
    RichText::from(text.to_string()).monospace()
}

pub fn mono_strong<T: ToString>(text: T) -> RichText {
    RichText::from(text.to_string()).monospace().strong()
}

pub fn error_text<T: ToString>(text: T) -> RichText {
    RichText::from(text.to_string())
        .color(Color32::RED)
        .background_color(Color32::BLACK)
        .monospace()
}

pub fn text_manip_menu(ui: &mut Ui, text: &mut String) {
    ui.menu_button("+", |ui| {
        if ui.button("Remove Whitespace").clicked() {
            *text = text.split_whitespace().collect();
        }
        if ui.button("UPPERCASE").clicked() {
            *text = text.to_uppercase();
        }
        if ui.button("lowercase").clicked() {
            *text = text.to_lowercase();
        }
    });
}

pub fn control_string(ui: &mut egui::Ui, string: &mut String) -> egui::Response {
    ui.add(egui::TextEdit::singleline(string).font(TextStyle::Monospace))
}

pub fn input_alphabet(ui: &mut egui::Ui, alphabet: &mut String) {
    ui.label("Alphabet");
    ui.add(egui::TextEdit::singleline(alphabet).font(TextStyle::Monospace));
}

pub fn text_edit(ui: &mut egui::Ui, text: &mut String) {
    ui.add(egui::TextEdit::singleline(text).font(TextStyle::Monospace));
}

pub fn letter_grid(ui: &mut egui::Ui, n_rows: usize, n_cols: usize, text: &String) {
    let symbols = str_to_char_grid(text, '\0', '\0');
    let grid = Grid::from_cols(symbols, n_rows, n_cols);

    egui::Grid::new("letter_grid").show(ui, |ui| {
        for n in 0..grid.num_rows() {
            ui.spacing_mut().item_spacing.x = 0.0;
            let row = grid.get_row(n);
            for c in row {
                let character = mono(*c.contents().unwrap()); // RichText::from(String::from(*c.contents().unwrap())).monospace();
                ui.add_sized([0.0, 0.0], Label::new(character));
            }
            ui.end_row()
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
            columns[col].label(mono_strong(&pair).size(18.0));
            ctr += 1;
            if ctr % nrows == 0 {
                col += 1
            }
        }
    });
}

pub fn randomize_reset(ui: &mut egui::Ui, cipher_frame: &mut dyn CipherFrame) {
    if ui.button("Randomize").clicked() {
        cipher_frame.randomize()
    }
    if ui.button("Reset").clicked() {
        cipher_frame.reset()
    }
}

// pub fn code_button_columns(
//     nrows: usize,
//     ncols: usize,
//     ui: &mut egui::Ui,
//     target: &mut String,
//     space: &str,
//     iter: Box<dyn Iterator<Item = (char, &str)> + '_>,
// ) {
//     ui.columns(ncols, |columns| {
//         let mut ctr = 0;
//         let mut col = 0;
//         for (c, code) in iter {
//             let pair = format!("{}  {} ", c, code);
//             if columns[col].button(&pair).clicked() {
//                 if !target.is_empty() {
//                     target.push_str(space);
//                 }
//                 target.push_str(code)
//             }
//             ctr += 1;
//             if ctr % nrows == 0 {
//                 col += 1
//             }
//         }
//     });
// }

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
