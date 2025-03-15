use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::traits::Cipher;
use ciphers::transposition::Amsco;
use eframe::egui::Ui;
use rand::{thread_rng, Rng};
use utils::grid::Grid;
use utils::preset_alphabet::Alphabet;
use utils::text_functions::{filter_string, random_string_sample_replace};

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
    fn build_example_grid(&mut self) {
        let groups = if let Some(groups) = self.cipher.groups(&self.example) {
            groups
        } else {
            self.example_grid = Grid::<String>::new_default(1, 1);
            return ();
        };
        let n_cols = self.cipher.key.len();
        let n_rows = num::Integer::div_ceil(&groups.len(), &n_cols);
        let mut groups_iter = groups.into_iter();

        self.example_grid = Grid::<String>::new_default(n_rows, n_cols);

        for k in self.cipher.key_ranks.iter() {
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

    fn assign_key(&mut self) {
        filter_string(&mut self.key_string, &self.alphabet_string);
        self.cipher
            .assign_key(&self.key_string, &self.alphabet_string)
            .unwrap() // justified by filtering of key_string
    }
}

impl CipherFrame for AmscoFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/ciphers/src/transposition/amsco.rs",
        );
        ui.add_space(8.0);

        ui.randomize_reset_cipher(self);
        ui.add_space(16.0);

        ui.subheading("Alphabet");
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.assign_key();
            self.build_example_grid();
        }

        ui.subheading("Keyword");
        if ui.control_string(&mut self.key_string).changed() {
            self.assign_key();
            self.build_example_grid();
        };

        ui.add_space(16.0);

        ui.group(|ui| {
            ui.subheading("Example Plaintext");
            if ui.control_string(&mut self.example).changed() {
                self.build_example_grid();
            };

            ui.add_space(8.0);

            ui.subheading("Example Grid");
            egui::Grid::new("amsco_grid")
                .num_columns(self.example_grid.num_cols())
                .min_col_width(20.0)
                .max_col_width(20.0)
                .striped(true)
                .show(ui, |ui| {
                    for letter in self.key_string.chars() {
                        ui.mono_strong(letter);
                    }
                    ui.end_row();
                    for digit in self.cipher.key.iter() {
                        ui.mono_strong(digit);
                    }
                    ui.end_row();
                    for row in 0..self.example_grid.num_rows() {
                        for c in self.example_grid.get_row(row) {
                            ui.mono(c);
                        }
                        ui.end_row();
                    }
                });

            ui.add_space(8.0);
        });

        ui.add_space(16.0);
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        let n_chars = rng.gen_range(6..10);

        self.key_string = random_string_sample_replace(&self.alphabet_string, n_chars, &mut rng);

        self.assign_key()
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
