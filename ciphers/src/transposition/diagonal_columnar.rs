use crate::traits::Cipher;
use utils::{
    errors::GeneralError,
    grid::{Grid, Symbol, BLOCK, EMPTY},
    text_functions::{rank_str, rank_vec, StringRankError},
};

pub struct DiagonalColumnar {
    pub key: Vec<usize>,
    pub key_ranks: Vec<usize>,
}

impl DiagonalColumnar {
    pub fn assign_key(&mut self, keyword: &str, alphabet: &str) -> Result<(), StringRankError> {
        self.key = rank_str(keyword, alphabet)?;
        self.key_ranks = rank_vec(&self.key);
        Ok(())
    }

    pub fn build_grid(&self, text_length: usize) -> Grid<Symbol<char>> {
        let n_cols = self.key.len();
        let n_rows = num::Integer::div_ceil(&text_length, &self.key.len());
        let mut g: Grid<Symbol<char>> = Grid::new_empty(n_rows, n_cols);

        let mut disruption_ctr = 0;
        let mut disruption_start = self.key.iter().position(|n| *n == disruption_ctr).unwrap();
        for row in 0..n_rows {
            g.get_row_mut(row)
                .skip(disruption_start)
                .for_each(|l| *l = Symbol::Blocked);

            if disruption_start >= n_cols {
                disruption_ctr += 1;
                disruption_start = match self.key.iter().position(|n| *n == disruption_ctr) {
                    Some(n) => n,
                    None => break,
                };
                continue;
            }
            disruption_start += 1;
        }
        g
    }

    fn fill_grid(&self, mut g: Grid<Symbol<char>>, text: &str) -> Grid<Symbol<char>> {
        // Fill the Empty cells left to right and top to bottom
        let mut symbols = text.chars();
        for cell in g.get_rows_mut() {
            if cell.is_empty() {
                match symbols.next() {
                    Some(c) => *cell = Symbol::Filled(c),
                    None => break,
                }
            }
        }

        // Fill the Blocked cells left to right and top to bottom
        for cell in g.get_rows_mut() {
            if cell.is_blocked() {
                match symbols.next() {
                    Some(c) => *cell = Symbol::Filled(c),
                    None => break,
                }
            }
        }
        g
    }

    pub fn display_page(&self, text_length: usize) -> String {
        self.build_grid(text_length).to_string()
    }

    pub fn display_page_filled(&self, text: &str) -> String {
        let text_length = text.chars().count();
        let g = self.fill_grid(self.build_grid(text_length), text);
        g.to_string()
    }
}

impl Default for DiagonalColumnar {
    fn default() -> Self {
        Self {
            key: Vec::new(),
            key_ranks: Vec::new(),
        }
    }
}

impl Cipher for DiagonalColumnar {
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        let text_length = text.chars().count();

        let g = self.fill_grid(self.build_grid(text_length), text);

        let mut out = String::with_capacity(text.len());
        for k in self.key_ranks.iter() {
            let mut s: String = g.get_col(*k).map(|sym| sym.to_char()).collect();
            s = s.replace(EMPTY, "");
            s = s.replace(BLOCK, "");
            out.push_str(&s);
        }

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
        let text_length = text.chars().count();
        let n_cols = self.key.len();
        let n_rows = num::Integer::div_ceil(&text_length, &self.key.len());

        let disrupted_grid = self.fill_grid(self.build_grid(text_length), text);
        let mut empty_grid: Grid<Symbol<char>> = Grid::new_empty(n_rows, n_cols);

        // Find where symbols stop being entered into the disrupted grid so the empty grid can be blocked in the remaining places
        // I have no idea how this work the way it is written.
        //let n_inner = text_length - disrupted_grid.num_empty();
        disrupted_grid
            .get_rows()
            .zip(empty_grid.get_rows_mut())
            .filter(|(a, _)| a.is_blocked())
            //.skip(n_inner)
            .for_each(|(_, b)| *b = Symbol::Blocked);

        // println!("{}", empty_grid.to_string());

        // Fill the grid by columns
        let mut symbols = text.chars();
        for n in self.key_ranks.iter() {
            let column = empty_grid.get_col_mut(*n);
            for cell in column {
                if cell.is_empty() {
                    match symbols.next() {
                        Some(c) => *cell = Symbol::Filled(c),
                        None => break,
                    }
                }
            }
        }

        let mut out = String::new();

        // Read the outer part of the disrupted sections
        let mut disruption_ctr = 0;
        let mut disruption_start = self.key.iter().position(|n| *n == disruption_ctr).unwrap();
        for row in 0..empty_grid.num_rows() {
            empty_grid
                .get_row_mut(row)
                .take(disruption_start)
                .for_each(|l| {
                    out.push(l.to_char());
                    *l = Symbol::Blocked;
                });

            if disruption_start >= empty_grid.num_cols() {
                disruption_ctr += 1;
                disruption_start = match self.key.iter().position(|n| *n == disruption_ctr) {
                    Some(n) => n,
                    None => break,
                };
                continue;
            }
            disruption_start += 1;
        }

        // Read the inner parts of the disrupted sections
        for row in 0..empty_grid.num_rows() {
            empty_grid
                .get_row(row)
                .filter(|cell| !cell.is_blocked())
                .for_each(|cell| out.push(cell.to_char()))
        }

        Ok(out)
    }
}

#[cfg(test)]
mod diagonal_columnar_tests {
    use utils::preset_alphabet::Alphabet;

    use super::*;

    const PLAINTEXT: &'static str = "WHENINTHECOURSEOFHUMANEVENTSITBECOMESNECESSARYFORONEPEOPLETODISSOLVETHEPOLITICALBANDSWHICHHAVECONNECTEDTHEMWITHANOTHERANDTOASSUMEAMONGTHEPOWERSOFTHEEARTHTHESEPARATEANDEQUALSTATIONTOWHICHTHELAWSOFNATUREANDOFNATURESGODENTITLETHEMADECENTRESPECTTOTHEOPINIONSOFMANKINDREQUIRESTHATTHEYSHOULDDECLARETHECAUSESWHICHIMPELTHEMTOTHESEPARATION";
    const CIPHERTEXT: &'static str = "NTOONISAEPDVLAHVCWODFDRATOOASAEUTNCWTTPFUOLEDCECNOSCTOCEPITIOSKUHEODECRHSHHMPNTETTEUFETNRTLIEINIETITTMRETYWFRERCEATHSHHAIRDITMHVBEYRPSTTTCCETHOAESTSUTTPEASIOTOEEROETHAEEECFEETHIHOODNEANQTHHLDHATUWCIHFMSANWNHRUNOERPESEADHNHHRNSUEOHEHTAEDATWEATRNNEENCEASESNOOLOBWAEMNNONIMGPSEELNQSOIALOAOTGHIESMTMSOETOPLSHNEAASIMANEREHTAELIHLEUEDAS";

    #[test]
    fn display_test() {
        let mut cipher = DiagonalColumnar::default();
        cipher
            .assign_key("RUTABEGA", Alphabet::BasicLatin.slice())
            .unwrap();
        println!("{}", cipher.display_page(PLAINTEXT.chars().count()));
        println!("{}", cipher.display_page_filled(PLAINTEXT));
    }

    #[test]
    fn encrypt_test() {
        let mut cipher = DiagonalColumnar::default();
        cipher
            .assign_key("RUTABEGA", Alphabet::BasicLatin.slice())
            .unwrap();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = DiagonalColumnar::default();
        cipher
            .assign_key("RUTABEGA", Alphabet::BasicLatin.slice())
            .unwrap();
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
