use eframe::egui;
use eframe::egui::Slider;
use super::View;
use super::generic_components::*;
use crate::ciphers::Grille;
 
 
fn cell_button(grille: &mut Grille, x: usize, y: usize, ui: &mut eframe::egui::Ui) {
    let cell = grille.grid[(x,y)];
    if ui.button(RichText::from(cell.to_char())).clicked() {
        if cell.is_blocked() {
            self.empty_cell((x,y));
        } else if cell.is_empty() {
            self.block_cell((x,y));
        }
    };
}
 
impl View for Grille {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
 
        randomize_reset(ui, self);
        ui.add_space(16.0);
 
        ui.label("Null Alphabet");
        //TextEdit
        ui.add_space(16.0);
 
        ui.label("Rows");
        ui.horizontal(|ui| {
            if ui.button("-").clicked() {
                self.remove_row();
            };
            ui.label(String::from(self.grid.num_rows()));
            if ui.button("+").clicked() {
                self.add_row();
            };
        });
        ui.add_space(16.0);
 
        ui.label("Columns");
        ui.horizontal(|ui| {
            if ui.button("-").clicked().clicked() {
                self.remove_col();
            };
            ui.label(String::from(self.grid.num_rows()));
            if ui.button("+").clicked().clicked() {
                self.add_col();
            };
        });
        ui.add_space(16.0);
 
        for x in 0..self.num_rows() {
            ui.horizontal(|ui| {
                for y in 0..self.num_cols() {
                    cell_button(self, x,y,ui);
                }
            });
        }
    }
}