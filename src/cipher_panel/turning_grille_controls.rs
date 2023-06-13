use ciphers::{transposition::TurningGrille, Cipher};
use egui::{TextStyle, Ui};
use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};

use crate::ui_elements::randomize_reset;

use super::CipherFrame;

#[derive(Default)]
pub struct TurningGrilleFrame {
    cipher: TurningGrille,
}

fn cell_button(grille: &mut TurningGrille, x: usize, y: usize, ui: &mut eframe::egui::Ui) {
    let cell = grille.grid[(x, y)];
    if ui.button(cell.to_char().to_string()).clicked() {
        ()
    };
}

impl CipherFrame for TurningGrilleFrame {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Adjust Size");
        ui.horizontal(|ui| {
            if ui.button("-").clicked() {
                self.cipher.decrease_size()
            };
            ui.label(format!("{}", self.cipher.grid.num_rows()));
            if ui.button("+").clicked() {
                self.cipher.increase_size()
            };
        });
        ui.add_space(16.0);

        ui.label("Keys");
        ui.label(format!(
            "The numbers from 0 to {} should all be used exactly once among the keys",
            self.cipher.subgrille_size() - 1
        ));
        ui.text_edit_singleline(&mut self.cipher.key_strings[0]);
        ui.text_edit_singleline(&mut self.cipher.key_strings[1]);
        ui.text_edit_singleline(&mut self.cipher.key_strings[2]);
        ui.text_edit_singleline(&mut self.cipher.key_strings[3]);

        if ui.button("Build Grid").clicked() {
            match self.cipher.build_key() {
                Ok(_) => (),
                Err(e) => *errors = e.to_string(),
            }
            match self.cipher.build_grid() {
                Ok(_) => (),
                Err(e) => errors.push_str(&e.to_string()),
            }
        };

        if ui.button("rotate").clicked() {
            self.cipher.grid.rotate()
        }

        ui.spacing_mut().item_spacing = (2.0, 2.0).into();
        ui.style_mut().override_text_style = Some(TextStyle::Monospace);
        for x in 0..self.cipher.grid.num_rows() {
            ui.horizontal(|ui| {
                for y in 0..self.cipher.grid.num_cols() {
                    cell_button(&mut self.cipher, x, y, ui);
                }
            });
        }
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        let mut nums = (0..self.cipher.subgrille_size()).collect_vec();
        nums.shuffle(&mut rng);
        let mut ctr = 0;

        for n in 0..4 {
            self.cipher.key_strings[n].clear();
            self.cipher.keys[n].clear();
        }

        for n in nums {
            self.cipher.keys[ctr].push(n);
            if !self.cipher.key_strings[ctr].is_empty() {
                self.cipher.key_strings[ctr].push_str(", ")
            }
            self.cipher.key_strings[ctr].push_str(&n.to_string());
            ctr = (ctr + 1) % 4
        }

        self.cipher.build_grid().unwrap();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
