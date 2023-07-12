use super::CipherFrame;

use crate::ui_elements::{control_string, mono, randomize_reset, subheading};
use ciphers::traits::Cipher;
use ciphers::transposition::Columnar;
use eframe::egui::Ui;
use rand::{thread_rng, Rng};
use utils::grid::Grid;
use utils::preset_alphabet::Alphabet;
use utils::text_functions::{filter_string, random_sample_replace};

pub struct ColumnarFrame {
    cipher: Columnar,
    alphabet_string: String,
    key_string: String,
    example: String,
}

impl Default for ColumnarFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatin.into(),
            key_string: Default::default(),
            example: Default::default(),
        }
    }
}

impl ColumnarFrame {
    fn assign_key(&mut self) {
        filter_string(&mut self.key_string, &self.alphabet_string);
        self.cipher
            .assign_key(&self.key_string, &self.alphabet_string)
            .unwrap() // justified by filtering of key_string
    }
}

impl CipherFrame for ColumnarFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label(subheading("Alphabet"));
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.assign_key()
        }

        ui.label(subheading("Keyword"));
        if control_string(ui, &mut self.key_string).changed() {
            self.assign_key()
        };

        ui.add_space(8.0);

        ui.collapsing("Example Grid", |ui| {
            ui.label(subheading("Text"));
            control_string(ui, &mut self.example);

            ui.label(subheading("Grid"));
            let n_cols = self.cipher.key.len();
            let n_rows = if n_cols > 0 {
                num::Integer::div_ceil(&self.example.chars().count(), &self.cipher.key.len())
            } else {
                0
            };
            let g = Grid::from_rows(self.example.chars().collect(), n_rows, n_cols);

            egui::Grid::new("columnar_grid")
                .num_columns(n_cols)
                .min_col_width(5.0)
                .max_col_width(5.0)
                .striped(true)
                .show(ui, |ui| {
                    for digit in self.cipher.key.iter() {
                        ui.label(mono(digit).strong());
                    }
                    ui.end_row();
                    for row in 0..n_rows {
                        for c in g.get_row(row) {
                            ui.label(mono(c));
                        }
                        ui.end_row();
                    }
                });
        });
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        let n_chars = rng.gen_range(6..10);

        self.key_string = random_sample_replace(&self.alphabet_string, n_chars, &mut rng);

        self.assign_key()
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
