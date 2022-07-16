use crate::ciphers::transposition::TurningGrille;

use super::{View, ViewableCipher, _generic_components::*};
use eframe::egui::{TextStyle, Ui};

impl ViewableCipher for TurningGrille {}

fn cell_button(grille: &mut TurningGrille, x: usize, y: usize, ui: &mut eframe::egui::Ui) {
    let cell = grille.grid[(x, y)];
    if ui.button(cell.to_char().to_string()).clicked() {
        ()
    };
}

impl View for TurningGrille {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Adjust Size");
        ui.horizontal(|ui| {
            if ui.button("-").clicked() {
                self.decrease_size()
            };
            ui.label(format!("{}", self.grid.num_rows()));
            if ui.button("+").clicked() {
                self.increase_size()
            };
        });
        ui.add_space(16.0);

        ui.label("Keys");
        ui.label(format!(
            "The numbers from 0 to {} should all be used exactly once among the keys",
            self.subgrille_size() - 1
        ));
        ui.text_edit_singleline(&mut self.key_strings[0]);
        ui.text_edit_singleline(&mut self.key_strings[1]);
        ui.text_edit_singleline(&mut self.key_strings[2]);
        ui.text_edit_singleline(&mut self.key_strings[3]);

        if ui.button("Build Grid").clicked() {
            match self.build_key() {
                Ok(_) => (),
                Err(e) => *errors = e.to_string(),
            }
            match self.build_grid() {
                Ok(_) => (),
                Err(e) => errors.push_str(&e.to_string()),
            }
        };

        if ui.button("rotate").clicked() {
            self.grid.rotate()
        }

        ui.spacing_mut().item_spacing = (2.0, 2.0).into();
        ui.style_mut().override_text_style = Some(TextStyle::Monospace);
        for x in 0..self.grid.num_rows() {
            ui.horizontal(|ui| {
                for y in 0..self.grid.num_cols() {
                    cell_button(self, x, y, ui);
                }
            });
        }
    }
}
