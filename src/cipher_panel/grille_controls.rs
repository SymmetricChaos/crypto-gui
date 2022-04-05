use eframe::egui::{TextStyle, TextEdit};
use super::View;
use super::generic_components::*;
use crate::ciphers::Grille;
use eframe::egui::Ui;
use rand::prelude::StdRng;
 
fn cell_button(grille: &mut Grille, x: usize, y: usize, ui: &mut eframe::egui::Ui) {
    let cell = grille.grid[(x,y)];
    if ui.button(cell.to_char().to_string()).clicked() {
        if cell.is_blocked() {
            grille.grid.empty_cell((x,y));
        } else if cell.is_empty() {
            grille.grid.block_cell((x,y));
        }
    };
}

impl View for Grille {
    fn ui(&mut self, ui: &mut Ui, rng: &mut StdRng) {
 
        randomize_reset(ui, self, rng);
        ui.add_space(16.0);

        ui.checkbox(&mut self.use_nulls, "Use Nulls?");
        if self.use_nulls {
            ui.label("Null Alphabet");
            ui.add(TextEdit::singleline(&mut self.null_alphabet).font(TextStyle::Monospace));
        }
        ui.add_space(16.0);

        ui.label("Rows");
        ui.horizontal(|ui| {
            if ui.button("-").clicked() {
                self.grid.del_row();
            };
            ui.label(format!("{}",self.grid.num_rows()));
            if ui.button("+").clicked() {
                self.grid.add_row();
            };
        });
        ui.add_space(10.0);

        ui.label("Columns");
        ui.horizontal(|ui| {
            if ui.button("-").clicked() {
                self.grid.del_col();
            };
            ui.label(format!("{}",self.grid.num_cols()));
            if ui.button("+").clicked() {
                self.grid.add_col();
            };
        });
        ui.add_space(16.0);

        ui.spacing_mut().item_spacing = (2.0,2.0).into();
        ui.style_mut().override_text_style = Some(TextStyle::Monospace);
        for x in 0..self.grid.num_rows() {
            ui.horizontal(|ui| {
                for y in 0..self.grid.num_cols() {
                    cell_button(self, x,y,ui);
                }
            });
        }
    }
}