use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{transposition::TurningGrille, Cipher};
use egui::Ui;
use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};
use utils::preset_alphabet::Alphabet;

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

// fn cell_button(grille: &mut TurningGrille, x: usize, y: usize, ui: &mut eframe::egui::Ui) {
//     let cell = grille.grid[(x, y)];
//     if ui.button(cell.to_char().to_string()).clicked() {
//         ()
//     };
// }

impl CipherFrame for TurningGrilleFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.subheading("Keys");
        ui.label(format!(
            "The numbers from 0 to {} should all be used exactly once among the keys",
            self.cipher.subgrille_size() - 1
        ));

        for (i, name) in ["Upper Left ", "Lower Left ", "Lower Right", "Upper Right"]
            .into_iter()
            .enumerate()
        {
            ui.horizontal(|ui| {
                ui.mono(name);
                if ui.control_string(&mut self.key_strings[i]).changed() {
                    match self.cipher.build_key(&self.key_strings) {
                        Ok(_) => (),
                        Err(e) => {
                            ui.error_text(e);
                        }
                    }
                    match self.cipher.build_grid() {
                        Ok(_) => (),
                        Err(e) => {
                            ui.error_text(e);
                        }
                    }
                }
            });
        }

        egui::Grid::new("turning_grille_grid")
            .num_columns(self.cipher.grille_width())
            .min_col_width(2.5)
            .max_col_width(2.5)
            .spacing(egui::Vec2::from((3.0, 2.0)))
            .striped(true)
            .show(ui, |ui| {
                for row in 0..self.cipher.grille_width() {
                    for c in self.cipher.grid.get_row(row) {
                        ui.label(c.to_char().to_string());
                    }
                    ui.end_row();
                }
            });

        ui.horizontal(|ui| {
            if ui.button("-").clicked() {
                self.cipher.decrease_size()
            };
            ui.label(format!("{}", self.cipher.grid.num_rows()));
            if ui.button("+").clicked() {
                self.cipher.increase_size()
            };
            ui.add_space(6.0);
            if ui.button("rotate").clicked() {
                self.cipher.grid.rotate()
            };
            ui.add_space(6.0);
            ui.copy_to_clipboard(self.cipher.grid.to_string());
        });

        ui.add_space(16.0);
        ui.subheading("Letters to Use as Nulls");
        if ui.control_string(&mut self.null_alphabet_string).changed() {
            self.cipher.assign_null_alphabet(&self.null_alphabet_string);
        };
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
