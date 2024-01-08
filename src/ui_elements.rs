use crate::cipher_panel::CipherFrame;
use codes::binary_to_text::BinaryToTextMode;
use eframe::egui::RichText;
use egui::{Color32, DragValue, Response, TextStyle, Ui};
use egui_extras::{Column, TableBuilder};
// use itertools::Itertools;
use rngs::ClassicRng;
use std::fmt::Display;
use utils::text_functions::filter_string;

pub trait UiElements {
    // A label with larger text
    fn subheading<T: ToString>(&mut self, text: T) -> Response;
    // Label with monospaced text
    fn mono<T: ToString>(&mut self, text: T) -> Response;
    // Label with strong monospaced text
    fn mono_strong<T: ToString>(&mut self, text: T) -> Response;
    // Label with red monospaced text on a black background
    fn error_text<T: ToString>(&mut self, text: T) -> Response;
    //A monospaced TextEdit that does not clip the text length
    fn control_string(&mut self, string: &mut String) -> Response;
    // Label with monospaced text and a black background, looks similar to control_string
    fn false_control_string<T: ToString>(&mut self, text: T) -> Response;
    // Buttons for Randomize and Reset
    fn randomize_reset(&mut self, cipher_frame: &mut dyn CipherFrame);
    // Slider variant that has a position at some character index of a &str
    fn string_slider(&mut self, string: &str, position: &mut usize) -> Response;
    // Button showing a clipboard that copies some text to the clipboard
    fn copy_to_clipboard<S: ToString>(&mut self, text: S);
    // Scrollable table with two columns and any number of rows
    fn two_column_table<S: ToString, T: ToString>(
        &mut self,
        left_label: &str,
        right_label: &str,
        iter: Box<dyn Iterator<Item = (S, T)> + '_>,
    );
    fn fill_code_columns<S: Display, T: Display>(
        &mut self,
        nrows: usize,
        ncols: usize,
        iter: Box<dyn Iterator<Item = (S, T)> + '_>,
    );
    fn binary_to_text_input_mode(&mut self, current_value: &mut BinaryToTextMode);
}

impl UiElements for Ui {
    fn subheading<T: ToString>(&mut self, text: T) -> Response {
        self.label(RichText::from(text.to_string()).size(16.0))
    }

    fn mono<T: ToString>(&mut self, text: T) -> Response {
        self.label(RichText::from(text.to_string()).monospace())
    }

    fn mono_strong<T: ToString>(&mut self, text: T) -> Response {
        self.label(RichText::from(text.to_string()).monospace().strong())
    }

    fn error_text<T: ToString>(&mut self, text: T) -> Response {
        self.label(
            RichText::from(text.to_string())
                .color(Color32::RED)
                .background_color(Color32::BLACK)
                .monospace(),
        )
    }

    fn control_string(&mut self, string: &mut String) -> Response {
        self.add(
            egui::TextEdit::singleline(string)
                .font(TextStyle::Monospace)
                .clip_text(false),
        )
    }

    fn false_control_string<T: ToString>(&mut self, text: T) -> Response {
        self.label(
            RichText::from(text.to_string())
                .monospace()
                .background_color(Color32::BLACK),
        )
    }

    fn randomize_reset(&mut self, cipher_frame: &mut dyn CipherFrame) {
        if self.button("Randomize").clicked() {
            cipher_frame.randomize()
        }
        if self.button("Reset").clicked() {
            cipher_frame.reset()
        }
    }

    fn string_slider(&mut self, string: &str, position: &mut usize) -> Response {
        self.add(
            DragValue::new(position)
                .clamp_range(0..=string.chars().count() - 1)
                .custom_formatter(|n, _| {
                    let n = n as usize;
                    string.chars().nth(n).unwrap().to_string()
                })
                .custom_parser(|s| {
                    if s.is_empty() {
                        Some(0.0)
                    } else {
                        let c = s.chars().next().unwrap();
                        string.chars().position(|x| x == c).map(|n| n as f64)
                    }
                })
                .speed(0.2),
        )
    }

    fn fill_code_columns<T: Display, S: Display>(
        &mut self,
        nrows: usize,
        ncols: usize,
        iter: Box<dyn Iterator<Item = (T, S)> + '_>,
    ) {
        self.columns(ncols, |columns| {
            let mut ctr = 0;
            let mut col = 0;
            for (c, code) in iter {
                let pair = format!("{}  {} ", c, code);
                columns[col].label(RichText::from(&pair).monospace().strong().size(18.0));
                ctr += 1;
                if ctr % nrows == 0 {
                    col += 1
                }
            }
        })
    }

    fn binary_to_text_input_mode(&mut self, current_value: &mut BinaryToTextMode) {
        self.label("Encoding Mode");
        self.selectable_value(current_value, BinaryToTextMode::Hex, "Hex")
            .on_hover_text("interpret input as hexcode");
        self.selectable_value(current_value, BinaryToTextMode::Utf8, "UTF-8")
            .on_hover_text("convert text to raw bytes");
    }

    fn copy_to_clipboard<S: ToString>(&mut self, text: S) {
        if self
            .button("📋")
            .on_hover_text("copy to clipboard")
            .clicked()
        {
            self.output_mut(|o| o.copied_text = text.to_string())
        }
    }

    fn two_column_table<S: ToString, T: ToString>(
        &mut self,
        left_label: &str,
        right_label: &str,
        iter: Box<dyn Iterator<Item = (S, T)> + '_>,
    ) {
        let table = TableBuilder::new(self)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::initial(70.0).range(20.0..=300.0))
            .column(Column::remainder())
            .min_scrolled_height(0.0);

        table
            .header(30.0, |mut header| {
                header.col(|ui| {
                    ui.strong(RichText::new(left_label).size(20.0));
                });
                header.col(|ui| {
                    ui.strong(RichText::new(right_label).size(20.0));
                });
            })
            .body(|mut body| {
                for (left, right) in iter {
                    body.row(20.0, |mut row| {
                        row.col(|ui| {
                            ui.label(RichText::new(left.to_string()).size(18.0));
                        });

                        row.col(|ui| {
                            ui.label(RichText::new(right.to_string()).size(18.0));
                        });
                    });
                }
            });
    }
}

// pub fn subheading<T: ToString>(text: T) -> RichText {
//     RichText::from(text.to_string()).size(16.0)
// }

// pub fn mono<T: ToString>(text: T) -> RichText {
//     RichText::from(text.to_string()).monospace()
// }

// pub fn mono_strong<T: ToString>(text: T) -> RichText {
//     RichText::from(text.to_string()).monospace().strong()
// }

// pub fn error_text<T: ToString>(text: T) -> RichText {
//     RichText::from(text.to_string())
//         .color(Color32::RED)
//         .background_color(Color32::BLACK)
//         .monospace()
// }

// pub fn control_string(ui: &mut egui::Ui, string: &mut String) -> egui::Response {
//     ui.add(
//         egui::TextEdit::singleline(string)
//             .font(TextStyle::Monospace)
//             .clip_text(false),
//     )
// }

pub fn randomize_reset(ui: &mut egui::Ui, cipher_frame: &mut dyn CipherFrame) {
    if ui.button("Randomize").clicked() {
        cipher_frame.randomize()
    }
    if ui.button("Reset").clicked() {
        cipher_frame.reset()
    }
}

pub fn string_slider(ui: &mut Ui, string: &str, position: &mut usize) -> Response {
    ui.add(
        DragValue::new(position)
            .clamp_range(0..=string.chars().count() - 1)
            .custom_formatter(|n, _| {
                let n = n as usize;
                string.chars().nth(n).unwrap().to_string()
            })
            .custom_parser(|s| {
                if s.is_empty() {
                    Some(0.0)
                } else {
                    let c = s.chars().next().unwrap();
                    string.chars().position(|x| x == c).map(|n| n as f64)
                }
            })
            .speed(0.2),
    )
}

pub fn filter_and_parse_u32(number: &mut u32, string: &mut String) {
    filter_string(string, &"0123456789");
    if string.is_empty() {
        *string = String::from("0");
        *number = 0;
    }
    *number = match string.parse() {
        Ok(n) => n,
        Err(_) => {
            *string = u32::MAX.to_string();
            u32::MAX
        }
    }
}

pub fn filter_and_parse_u64(number: &mut u64, string: &mut String) {
    filter_string(string, &"0123456789");
    if string.is_empty() {
        *string = String::from("0");
        *number = 0;
    }
    *number = match string.parse() {
        Ok(n) => n,
        Err(_) => {
            *string = u64::MAX.to_string();
            u64::MAX
        }
    }
}

pub fn generate_random_nums_box(
    ui: &mut Ui,
    rng: &mut dyn ClassicRng,
    n_random: &mut usize,
    randoms: &mut String,
) {
    ui.horizontal(|ui| {
        if ui.button("Random Numbers").clicked() {
            for _ in 0..*n_random {
                if !randoms.is_empty() {
                    randoms.push_str(", ");
                }
                randoms.push_str(&rng.next_u32().to_string());
            }
        }
        ui.add(DragValue::new(n_random).clamp_range(1..=10))
    });

    ui.text_edit_multiline(randoms);
}

pub fn binary_to_text_input_mode(ui: &mut egui::Ui, current_value: &mut BinaryToTextMode) {
    ui.label("Encoding Mode");
    ui.selectable_value(current_value, BinaryToTextMode::Hex, "Hex")
        .on_hover_text("interpret input as hexcode");
    ui.selectable_value(current_value, BinaryToTextMode::Utf8, "UTF-8")
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
            columns[col].label(RichText::from(&pair).monospace().strong().size(18.0));
            ctr += 1;
            if ctr % nrows == 0 {
                col += 1
            }
        }
    });
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

// pub fn letter_grid(ui: &mut egui::Ui, n_rows: usize, n_cols: usize, text: &String) {
//     let symbols = str_to_char_grid(text, '\0', '\0');
//     let grid = Grid::from_cols(symbols, n_rows, n_cols);
//     egui::Grid::new("letter_grid").show(ui, |ui| {
//         for n in 0..grid.num_rows() {
//             ui.spacing_mut().item_spacing.x = 0.0;
//             let row = grid.get_row(n);
//             for c in row {
//                 let character = mono(*c.contents().unwrap()); // RichText::from(String::from(*c.contents().unwrap())).monospace();
//                 ui.add_sized([0.0, 0.0], Label::new(character));
//             }
//             ui.end_row()
//         }
//     });
// }

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
