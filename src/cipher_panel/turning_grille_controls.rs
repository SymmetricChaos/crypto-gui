use ciphers::{transposition::TurningGrille, Cipher};
use egui::{TextStyle, Ui};
use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};
use utils::preset_alphabet::Alphabet;

use crate::ui_elements::{control_string, error_text, randomize_reset};

use super::CipherFrame;

pub struct TurningGrilleFrame {
    cipher: TurningGrille,
    null_alphabet_string: String,
    key_strings: [String; 4],
}

impl Default for TurningGrilleFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            null_alphabet_string: String::from(Alphabet::BasicLatin),
            key_strings: [
                "0, 1, 2, 3".into(),
                "4, 5, 6, 7".into(),
                "8, 9, 10, 11".into(),
                "12, 13, 14, 15".into(),
            ],
        }
    }
}

fn cell_button(grille: &mut TurningGrille, x: usize, y: usize, ui: &mut eframe::egui::Ui) {
    let cell = grille.grid[(x, y)];
    if ui.button(cell.to_char().to_string()).clicked() {
        ()
    };
}

impl CipherFrame for TurningGrilleFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Keys");
        ui.label(format!(
            "The numbers from 0 to {} should all be used exactly once among the keys",
            self.cipher.subgrille_size() - 1
        ));

        for (i, name) in ["Upper Left", "Lower Left", "Lower Right", "Upper Right"]
            .into_iter()
            .enumerate()
        {
            ui.horizontal(|ui| {
                ui.label(name);
                if control_string(ui, &mut self.key_strings[i]).changed() {
                    match self.cipher.build_key(&self.key_strings) {
                        Ok(_) => (),
                        Err(e) => {
                            ui.label(error_text(e.to_string()));
                        }
                    }
                    match self.cipher.build_grid() {
                        Ok(_) => (),
                        Err(e) => {
                            ui.label(error_text(e.to_string()));
                        }
                    }
                }
            });
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
        if ui.button("rotate").clicked() {
            self.cipher.grid.rotate()
        }

        ui.label("Letters to Use as Nulls");
        control_string(ui, &mut self.null_alphabet_string);
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
            self.key_strings[n].clear();
            self.cipher.keys[n].clear();
        }

        for n in nums {
            self.cipher.keys[ctr].push(n);
            if !self.key_strings[ctr].is_empty() {
                self.key_strings[ctr].push_str(", ")
            }
            self.key_strings[ctr].push_str(&n.to_string());
            ctr = (ctr + 1) % 4
        }

        self.cipher.build_grid().unwrap();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
