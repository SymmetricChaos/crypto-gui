use ciphers::{
    tactical::{
        rs44::{HEIGHT, LABELS, WIDTH},
        Rs44,
    },
    Cipher,
};
use egui::{Button, Color32, DragValue, RichText, TextStyle, Ui, Vec2};
use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};
use utils::grid::{Grid, Symbol};

use crate::ui_elements::UiElements;

use super::CipherFrame;

#[derive(Default)]
pub struct Rs44Frame {
    cipher: Rs44,
}

fn cell_button_char(grille: &Grid<char>, x: usize, y: usize, ui: &mut eframe::egui::Ui) {
    let cell = grille[(x, y)];
    let symbol = cell.to_string();
    ui.add_enabled(false, Button::new(symbol).frame(false));
}

impl CipherFrame for Rs44Frame {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/tactical/rs44.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.label("Start Column")
            .on_hover_text_at_pointer("the index of the column that is read first when encrypting");
        if ui
            .add(DragValue::new(&mut self.cipher.start_column).range(0..=24))
            .changed()
        {
            self.cipher.set_full_message_key();
        };

        ui.add_space(16.0);
        ui.label("Starting Cell").on_hover_text_at_pointer(
            "the coordinates of the cell where the text is written into the grid when encrypting",
        );
        ui.horizontal(|ui| {
            // The user changes the second field of the index with the x coordinate and the first field with the y coordinate
            // Grid index notation is flipped from the more familiar xy notation
            if ui
                .add(
                    DragValue::new(&mut self.cipher.start_cell.1)
                        .prefix("x: ")
                        .range(0..=24),
                )
                .changed()
            {
                self.cipher.set_full_message_key();
            };
            if ui
                .add(
                    DragValue::new(&mut self.cipher.start_cell.0)
                        .prefix("y: ")
                        .range(0..=23),
                )
                .changed()
            {
                self.cipher.set_full_message_key();
            };
            if self.cipher.stencil[self.cipher.start_cell].is_blocked() {
                ui.label(
                    RichText::new("Invalid Start Position")
                        .color(Color32::RED)
                        .background_color(Color32::BLACK)
                        .monospace(),
                );
            } else {
                ui.label(" ");
            }
        });

        // The Message Key area. Not needed for encryption.
        ui.add_space(16.0);
        ui.collapsing("Message Key", |ui| {
            ui.spacing_mut().item_spacing = (2.0, 2.0).into();
            ui.style_mut().override_text_style = Some(TextStyle::Monospace);

            ui.label("Letter Encyption Square").on_hover_text_at_pointer("each letter along the top can be written as any of the letters in the column below it");
            ui.horizontal(|ui| {
                for letter in ["a", "b", "c", "d", "e"] {
                    ui.add_enabled(false, Button::new(letter).frame(false));
                }
            });
            ui.horizontal(|ui| {
                for _ in 0..5 {
                    ui.add_enabled(false, Button::new("-").frame(false));
                }
            });
            for x in 0..self.cipher.message_key_maxtrix.num_rows() {
                ui.horizontal(|ui| {
                    for y in 0..self.cipher.message_key_maxtrix.num_cols() {
                        cell_button_char(&self.cipher.message_key_maxtrix, x, y, ui);
                    }
                });
            }

            ui.add_space(8.0);
            ui.label(
                RichText::new(format!("Message Key: {}", self.cipher.encrypted_message_key))
                    .strong()
                    .monospace(),
            ).on_hover_text_at_pointer("to be transmitted with the message so that the reciever can configure their own cipher");

            if ui.button("New Key").clicked() {
                self.cipher.set_full_message_key()
            }
        });

        ui.add_space(10.0);
        egui::Grid::new("control_rs44_grid")
            .spacing(Vec2 { x: -14.0, y: 2.0 })
            .num_columns(27)
            .show(ui, |ui| {
                // Position cursors on top
                ui.label(" ");
                ui.label(" ");
                for col in 0..25 {
                    if col == self.cipher.start_column {
                        ui.label(RichText::from("🡫").strong().size(24.0));
                    } else {
                        ui.label(" ");
                    }
                }
                ui.end_row();

                ui.label(" ");
                ui.label(" ");
                for col in 0..25 {
                    if col == self.cipher.start_cell.1 {
                        ui.label(RichText::from("🡫").strong().size(24.0));
                    } else {
                        ui.label(" ");
                    }
                }

                // labels
                ui.end_row();
                ui.label(" ");
                ui.label(" ");
                for l in self.cipher.xlabels.iter() {
                    ui.label(l.to_string());
                }
                // numbers
                ui.end_row();
                ui.label(" ");
                ui.label(" ");
                for n in self.cipher.column_nums.iter() {
                    ui.label((n + 1).to_string());
                }
                ui.end_row();

                for row in 0..24 {
                    if row == self.cipher.start_cell.0 {
                        ui.label(RichText::from("🡪").strong().size(24.0));
                    } else {
                        ui.label(" ");
                    }
                    ui.label(self.cipher.ylabels[row]);
                    for s in self.cipher.stencil.get_row(row) {
                        ui.label(RichText::new(s.to_char()).monospace().size(24.0));
                    }
                    ui.end_row();
                }
            });

        if ui.button("Copy Stencil to Clipboard").clicked() {
            ui.output_mut(|o| o.copied_text = self.cipher.stencil_to_text());
        }

        ui.text_edit_singleline(&mut self.cipher.imported_stencil);
        if ui.button("Import Stencil").clicked() {
            match self.cipher.text_to_stencil() {
                Ok(_) => (),
                Err(e) => *errors = e.to_string(),
            }
        }
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();

        // Randomize the matrix
        self.cipher.message_key_maxtrix.shuffle(&mut rng);

        // Randomize stencil
        self.cipher.stencil.apply(|_| Symbol::Blocked);
        let mut positions: Vec<usize> = (0..WIDTH).collect();
        for i in 0..HEIGHT {
            positions.shuffle(&mut rng);
            for n in &positions[0..10] {
                self.cipher.stencil[n + (i * WIDTH)] = Symbol::Empty;
            }
        }

        // Randomize labels
        self.cipher.column_nums.shuffle(&mut rng);
        self.cipher.xlabels.shuffle(&mut rng);
        self.cipher.ylabels = {
            let mut v = LABELS.clone();
            v.shuffle(&mut rng);
            v.iter()
                .take(HEIGHT)
                .map(|x| *x)
                .collect_vec()
                .try_into()
                .unwrap()
        };

        // Set the message key again
        self.cipher.set_full_message_key();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
