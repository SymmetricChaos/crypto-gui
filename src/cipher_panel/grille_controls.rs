use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{transposition::Grille, Cipher};
use egui::{TextStyle, Ui};
use rand::{rngs::StdRng, Rng, SeedableRng};
use utils::{grid::Symbol, preset_alphabet::Alphabet};

pub struct GrilleFrame {
    cipher: Grille,
    null_alphabet_string: String,
}

impl Default for GrilleFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            null_alphabet_string: Alphabet::BasicLatin.into(),
        }
    }
}

fn cell_button(grille: &mut Grille, x: usize, y: usize, ui: &mut eframe::egui::Ui) {
    let cell = grille.grid[(x, y)];
    if ui.button(cell.to_char().to_string()).clicked() {
        if cell.is_blocked() {
            grille.grid.empty_cell((x, y));
        } else if cell.is_empty() {
            grille.grid.block_cell((x, y));
        }
    };
}

impl CipherFrame for GrilleFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/transposition/grille.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.checkbox(&mut self.cipher.use_nulls, "Use Nulls");
        if self.cipher.use_nulls {
            if ui.control_string(&mut self.null_alphabet_string).changed() {
                self.cipher.assign_null_alphabet(&self.null_alphabet_string)
            }
        }
        ui.add_space(16.0);

        ui.subheading("Rows");
        ui.horizontal(|ui| {
            if ui.button("-").clicked() {
                self.cipher.grid.del_row();
            };
            ui.label(format!("{}", self.cipher.grid.num_rows()));
            if ui.button("+").clicked() {
                self.cipher.grid.add_row();
            };
        });
        ui.add_space(10.0);

        ui.subheading("Columns");
        ui.horizontal(|ui| {
            if ui.button("-").clicked() {
                self.cipher.grid.del_col();
            };
            ui.label(format!("{}", self.cipher.grid.num_cols()));
            if ui.button("+").clicked() {
                self.cipher.grid.add_col();
            };
        });
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.subheading("Grid");
            ui.copy_to_clipboard(self.cipher.grid.to_string());
        });
        ui.spacing_mut().item_spacing = (2.0, 2.0).into();
        ui.style_mut().override_text_style = Some(TextStyle::Monospace);
        for x in 0..self.cipher.grid.num_rows() {
            ui.horizontal(|ui| {
                for y in 0..self.cipher.grid.num_cols() {
                    cell_button(&mut self.cipher, x, y, ui);
                }
            });
        }
        ui.label(format!("{} empty cells", self.cipher.grid.num_empty()));
        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = StdRng::from_entropy();
        for cell in self.cipher.grid.get_rows_mut() {
            if rng.gen_bool(0.5) {
                *cell = Symbol::Empty;
            } else {
                *cell = Symbol::Blocked;
            }
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
