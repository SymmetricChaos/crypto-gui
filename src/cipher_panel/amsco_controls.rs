use super::CipherFrame;

use crate::ui_elements::{control_string, mono, randomize_reset};
use ciphers::traits::Cipher;
use ciphers::transposition::Amsco;
use eframe::egui::Ui;
use rand::{thread_rng, Rng};
use utils::functions::random_sample_replace;
use utils::grid::Grid;
use utils::preset_alphabet::Alphabet;

pub struct AmscoFrame {
    cipher: Amsco,
    alphabet_string: String,
    key_string: String,
    example: String,
    example_grid: Grid<String>,
}

impl Default for AmscoFrame {
    fn default() -> Self {
        Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatin.into(),
            key_string: Default::default(),
            example: Default::default(),
            example_grid: Grid::new_default(1, 1),
        }
    }
}

impl AmscoFrame {
    fn build_grid(&mut self) {
        let groups = self.cipher.groups(&self.example);
        let n_cols = self.cipher.key.len();
        let n_rows = num::Integer::div_ceil(&groups.len(), &n_cols);
        let mut groups_iter = groups.into_iter();

        self.example_grid = Grid::<String>::new_default(n_rows, n_cols);

        for k in self.cipher.key.iter() {
            for row in self.example_grid.get_col_mut(*k) {
                if let Some(a) = groups_iter.next() {
                    match a {
                        (c, None) => *row = c.to_string(),
                        (c1, Some(c2)) => *row = format!("{c1}{c2}"),
                    }
                }
            }
        }
    }
}

impl CipherFrame for AmscoFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        if control_string(ui, &mut self.alphabet_string).changed() {
            self.cipher
                .assign_key(&self.key_string, &self.alphabet_string);
            self.build_grid();
        }

        ui.label("Key Word");
        if control_string(ui, &mut self.key_string).changed() {
            self.cipher
                .assign_key(&self.key_string, &self.alphabet_string);
            self.build_grid();
        };

        ui.add_space(16.0);

        ui.label("Example");
        if control_string(ui, &mut self.example).changed() {
            self.build_grid();
        };

        ui.add_space(8.0);

        ui.label("Grid");
        egui::Grid::new("amsco_grid")
            .num_columns(self.example_grid.num_cols())
            .min_col_width(20.0)
            .max_col_width(20.0)
            .striped(true)
            .show(ui, |ui| {
                for digit in self.cipher.key.iter() {
                    ui.label(mono(digit).strong());
                }
                ui.end_row();
                for row in 0..self.example_grid.num_rows() {
                    for c in self.example_grid.get_row(row) {
                        ui.label(mono(c));
                    }
                    ui.end_row();
                }
            });
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        let n_chars = rng.gen_range(6..10);

        self.key_string = random_sample_replace(&self.alphabet_string, n_chars, &mut rng);

        self.cipher
            .assign_key(&self.key_string, &self.alphabet_string)
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
