use super::CipherFrame;

use crate::ui_elements::UiElements;
use ciphers::traits::Cipher;
use ciphers::transposition::Columnar;
use eframe::egui::Ui;
use rand::{thread_rng, Rng};
use utils::grid::Grid;
use utils::preset_alphabet::Alphabet;
use utils::text_functions::{filter_string, random_string_sample_replace};

pub struct ColumnarFrame {
    cipher: Columnar,
    alphabet_string: String,
    key_string: String,
    example: String,
    example_grid: Grid<char>,
}

impl Default for ColumnarFrame {
    fn default() -> Self {
        let mut f = Self {
            cipher: Default::default(),
            alphabet_string: Alphabet::BasicLatin.into(),
            key_string: String::from("CIPHER"),
            example: String::from("COLUMNARTRANSPOSITIONCIPHEREXAMPLE"),
            example_grid: Grid::new_default(1, 1),
        };
        f.assign_key();
        f.set_example();
        f
    }
}

impl ColumnarFrame {
    fn assign_key(&mut self) {
        filter_string(&mut self.key_string, &self.alphabet_string);
        self.cipher
            .assign_key(&self.key_string, &self.alphabet_string)
            .unwrap() // justified by filtering of key_string
    }

    fn set_example(&mut self) {
        if self.example.is_empty() || self.cipher.key.is_empty() {
            self.example_grid = Grid::new_default(1, 1);
            return;
        }
        let n_cols = self.cipher.key.len();
        let n_rows = if n_cols > 0 {
            num::Integer::div_ceil(&self.example.chars().count(), &self.cipher.key.len())
        } else {
            0
        };
        self.example_grid = Grid::from_rows(self.example.chars().collect(), n_rows, n_cols);
    }
}

impl CipherFrame for ColumnarFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.randomize_reset(self);
        ui.add_space(16.0);

        ui.subheading("Alphabet");
        if ui.control_string(&mut self.alphabet_string).changed() {
            self.assign_key();
            self.set_example();
        }

        ui.subheading("Keyword");
        if ui.control_string(&mut self.key_string).changed() {
            self.assign_key();
            self.set_example();
        };

        ui.add_space(16.0);

        ui.collapsing("Example", |ui| {
            ui.subheading("Text");
            if ui.control_string(&mut self.example).changed() {
                self.set_example();
            }
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.subheading("Grid");
                ui.copy_to_clipboard(self.example_grid.to_string());
            });
            ui.add_space(4.0);
            if self.example.is_empty() {
                ui.error_text("no plaintext provided")
            } else if self.key_string.is_empty() {
                ui.error_text("no key provided")
            } else {
                egui::Grid::new("columnar_grid")
                    .num_columns(self.example_grid.num_cols())
                    .min_col_width(5.0)
                    .max_col_width(5.0)
                    .striped(true)
                    .show(ui, |ui| {
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
                match self.cipher.encrypt(&self.example) {
                    Ok(t) => ui.mono(t),
                    Err(e) => ui.error_text(e),
                }
            }
        });
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        let n_chars = rng.gen_range(6..10);

        self.key_string = random_string_sample_replace(&self.alphabet_string, n_chars, &mut rng);

        self.assign_key();
        self.set_example();
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
