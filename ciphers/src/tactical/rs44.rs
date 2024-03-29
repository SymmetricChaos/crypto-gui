use crate::{errors::CipherError, traits::Cipher};
use itertools::Itertools;
use rand::{
    prelude::{SliceRandom, StdRng},
    Rng, SeedableRng,
};
use std::{iter::Chain, ops::Range};
use utils::{
    grid::{Grid, Symbol, BLOCK, EMPTY},
    preset_alphabet::Alphabet,
};

const DEFAULT_STENCIL: &'static str = "⬛⬜⬛⬛⬛⬛⬛⬜⬜⬛⬛⬜⬛⬜⬛⬜⬛⬛⬜⬜⬛⬛⬜⬜⬛⬛⬛⬜⬛⬜⬛⬛⬜⬛⬜⬜⬛⬛⬜⬛⬛⬛⬜⬛⬛⬜⬛⬜⬛⬜⬜⬛⬛⬜⬜⬜⬛⬛⬛⬛⬜⬛⬛⬜⬛⬜⬛⬜⬛⬜⬛⬛⬛⬜⬛⬛⬜⬛⬜⬛⬜⬜⬜⬛⬜⬛⬛⬜⬛⬛⬜⬛⬛⬜⬛⬛⬜⬛⬛⬛⬜⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬜⬛⬜⬜⬛⬜⬛⬛⬛⬛⬜⬜⬛⬜⬛⬛⬛⬜⬛⬛⬜⬛⬛⬜⬛⬜⬜⬜⬛⬛⬜⬜⬛⬛⬜⬛⬛⬜⬛⬜⬛⬛⬛⬜⬛⬛⬛⬛⬜⬛⬜⬛⬛⬜⬛⬛⬛⬜⬜⬜⬛⬛⬜⬜⬛⬛⬜⬛⬜⬛⬜⬛⬛⬜⬜⬛⬛⬛⬜⬜⬛⬛⬜⬛⬛⬜⬛⬜⬛⬛⬜⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬜⬜⬛⬛⬜⬛⬜⬜⬛⬛⬜⬜⬜⬛⬜⬜⬛⬜⬛⬜⬛⬛⬜⬜⬛⬜⬛⬛⬛⬜⬜⬛⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬜⬛⬛⬜⬜⬜⬛⬜⬛⬛⬜⬛⬛⬜⬜⬛⬛⬛⬜⬛⬜⬛⬜⬛⬜⬜⬛⬜⬛⬜⬛⬜⬛⬛⬛⬜⬛⬜⬛⬛⬜⬛⬜⬛⬛⬛⬜⬛⬛⬜⬛⬛⬜⬜⬛⬛⬜⬛⬜⬜⬛⬛⬛⬜⬛⬛⬜⬜⬜⬛⬜⬜⬜⬛⬛⬛⬛⬜⬛⬛⬛⬛⬜⬛⬛⬛⬜⬜⬛⬜⬛⬛⬛⬛⬛⬜⬛⬛⬛⬜⬜⬛⬜⬛⬛⬜⬜⬛⬛⬜⬜⬛⬛⬛⬜⬜⬛⬛⬛⬜⬜⬛⬛⬜⬜⬛⬛⬛⬜⬛⬛⬛⬛⬜⬛⬛⬛⬜⬜⬜⬜⬛⬛⬜⬜⬛⬜⬛⬛⬜⬛⬛⬜⬜⬛⬜⬛⬜⬛⬜⬛⬛⬛⬛⬛⬜⬛⬜⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬛⬜⬛⬜⬜⬜⬛⬛⬛⬜⬜⬛⬛⬜⬛⬛⬜⬜⬛⬜⬛⬛⬛⬜⬛⬛⬛⬛⬜⬛⬜⬜⬛⬛⬜⬛⬜⬜⬛⬜⬛⬛⬜⬛⬛⬜⬛⬛⬜⬜⬛⬛⬜⬜⬛⬛⬜⬛⬜⬛⬜⬛⬛⬛⬜⬛⬜⬛⬛⬜⬛⬛⬜⬜⬛⬛⬛⬛⬜⬜⬛⬛⬜⬛⬜⬛⬜⬛⬛⬛⬛⬜⬛⬜⬛⬜⬛⬛⬛⬜⬜⬛⬛⬛⬛⬜⬛⬛⬜⬜⬛⬜⬛⬜⬛⬜⬛⬛⬛⬛⬜⬜⬛⬜⬛⬛⬜⬛⬜⬜⬛⬛⬜⬛⬛⬜⬛⬜⬛⬛⬛⬛⬜⬛⬛⬛⬛⬜⬜⬛⬛⬜⬜⬜⬛⬛⬜⬜⬜⬛⬛⬛⬜⬛";

pub struct Rs44 {
    pub stencil: Grid<Symbol<char>>,
    pub column_nums: [usize; 25],
    pub xlabels: [&'static str; 25],
    pub ylabels: [&'static str; 24],
    pub start_cell: (usize, usize),
    pub start_column: usize,
    pub message_key_maxtrix: Grid<char>,
    pub hours: u8,
    pub minutes: u8,
    pub encrypted_message_key: String,
    pub imported_stencil: String,
}

impl Default for Rs44 {
    fn default() -> Self {
        let mut rng = StdRng::seed_from_u64(5920348976);

        // Build the stencile
        let mut stencil: Grid<Symbol<char>> = Grid::new_blocked(HEIGHT, WIDTH);
        for (i, c) in DEFAULT_STENCIL.chars().enumerate() {
            if c == EMPTY {
                stencil[i] = Symbol::Empty;
            }
        }
        let column_nums = [
            13, 1, 20, 10, 18, 14, 0, 7, 17, 9, 23, 2, 6, 11, 16, 19, 4, 12, 22, 15, 5, 3, 21, 24,
            8,
        ];
        let xlabels: [&str; WIDTH] = {
            let mut arr = LABELS.clone();
            arr.shuffle(&mut rng);
            arr
        };
        let ylabels: [&str; HEIGHT] = {
            let mut v = LABELS.clone();
            v.shuffle(&mut rng);
            v.iter()
                .take(HEIGHT)
                .map(|x| *x)
                .collect_vec()
                .try_into()
                .unwrap()
        };
        let message_key_maxtrix = {
            let mut g: Grid<char> = Grid::from_rows(
                Alphabet::BasicLatinNoJ
                    .chars()
                    .map(|c| c.to_ascii_lowercase())
                    .collect_vec(),
                5,
                5,
            );
            g.shuffle(&mut rng);
            g
        };

        Self {
            stencil,
            column_nums,
            xlabels,
            ylabels,
            start_cell: (0, 0),
            start_column: 0,
            message_key_maxtrix,
            hours: 0,
            minutes: 0,
            encrypted_message_key: String::new(),
            imported_stencil: String::new(),
        }
    }
}

pub const WIDTH: usize = 25;
pub const HEIGHT: usize = 24;
pub const LABELS: [&'static str; 25] = [
    "aa", "ba", "ca", "da", "ea", "ab", "bb", "cb", "db", "eb", "ac", "bc", "cc", "dc", "ec", "ad",
    "bd", "cd", "dd", "de", "ae", "be", "ce", "de", "ee",
];
pub const MESSAGE_LENGTH: usize = 240;
pub const GRID_SIZE: usize = 600;

impl Rs44 {
    fn label_letter_to_matrix_column(&self, c: char) -> usize {
        match c {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            _ => unreachable!("the only letter used for labels are 'abcde'"),
        }
    }

    pub fn encrypt_label_char(&self, c: char, rng: &mut StdRng) -> char {
        let row: usize = rng.gen_range(0..5);
        let col = self.label_letter_to_matrix_column(c);
        self.message_key_maxtrix[(row, col)]
    }

    pub fn set_full_message_key(&mut self) {
        self.encrypted_message_key.clear();
        let mut rng = StdRng::from_entropy();
        self.xlabels[self.start_cell.1].chars().for_each(|c| {
            self.encrypted_message_key
                .push(self.encrypt_label_char(c, &mut rng))
        });
        self.ylabels[self.start_cell.0].chars().for_each(|c| {
            self.encrypted_message_key
                .push(self.encrypt_label_char(c, &mut rng))
        });
        self.encrypted_message_key.push('-');
        self.xlabels[self.start_column].chars().for_each(|c| {
            self.encrypted_message_key
                .push(self.encrypt_label_char(c, &mut rng))
        });
    }

    fn col_num_to_col_idx(&self, n: usize) -> usize {
        self.column_nums
            .iter()
            .position(|x| &n == x)
            .expect("invalid column number supplied")
    }

    fn offset_col_nums(&self) -> Chain<Range<usize>, Range<usize>> {
        let start_idx = self.column_nums[self.start_column];
        (start_idx..25).chain(0..start_idx)
    }

    pub fn stencil_to_text(&self) -> String {
        self.stencil.get_rows().map(|c| c.to_char()).collect()
    }

    pub fn text_to_stencil(&mut self) -> Result<(), CipherError> {
        let mut vec = Vec::with_capacity(GRID_SIZE);
        let mut ctr = 0;
        for (n, c) in self.imported_stencil.chars().enumerate() {
            if c == EMPTY {
                vec.push(Symbol::Empty);
                ctr += 1;
            } else if c == BLOCK {
                vec.push(Symbol::Empty)
            } else {
                return Err(CipherError::Key(format!(
                    "The RS44 key can only be built from the symbols {} and {}",
                    EMPTY, BLOCK
                )));
            }
            if (n + 1) % 25 == 0 && ctr % 10 != 0 {
                return Err(CipherError::key(
                    "The RS44 stencil must have exactly 10 empty spaces in each row",
                ));
            }
        }
        if vec.len() != GRID_SIZE {
            return Err(CipherError::key(
                "The RS44 key must have exactly 600 positions defined",
            ));
        }
        self.stencil = Grid::from_rows(vec, 24, 25);
        self.imported_stencil.clear();
        Ok(())
    }

    fn bounds_check(&self) -> Result<(), CipherError> {
        match self.stencil.get(self.start_cell) {
            Some(s) => {
                if !s.is_empty() {
                    return Err(CipherError::key("starting cell must be an empty position"));
                } else {
                    ()
                }
            }
            None => return Err(CipherError::key("starting cell out of bounds")),
        }
        if self.start_column >= WIDTH {
            return Err(CipherError::key("starting column out of bounds"));
        }
        Ok(())
    }

    fn wrapping_iter(&self, n: usize) -> Chain<Range<usize>, Range<usize>> {
        (n..GRID_SIZE).chain(0..n)
    }
}

impl Cipher for Rs44 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        self.bounds_check()?;

        let mut output = String::with_capacity(text.len());

        let mut symbols = text.chars();
        let mut stencil = self.stencil.clone();
        let start = stencil.index_from_coord(self.start_cell).unwrap();

        for idx in self.wrapping_iter(start) {
            if stencil[idx].is_empty() {
                match symbols.next() {
                    Some(c) => stencil[idx] = Symbol::Filled(c),
                    None => break,
                }
            }
        }

        let positions = self.offset_col_nums().map(|n| self.col_num_to_col_idx(n));

        for col in positions {
            let s: String = stencil
                .get_col(col)
                .filter(|sym| sym.is_filled())
                .map(|sym| sym.to_char())
                .collect();
            output.push_str(&s);
        }

        Ok(output)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.bounds_check()?;

        let mut symbols = text.chars();
        let mut stencil = self.stencil.clone();

        // We must tag the cells that will be used for the message in order to
        // write into the columns correctly
        let start = stencil.index_from_coord(self.start_cell).unwrap();
        let mut temp_symbols = symbols.clone();
        for idx in self.wrapping_iter(start) {
            if stencil[idx].is_empty() {
                match temp_symbols.next() {
                    Some(_) => stencil[idx] = Symbol::Filled('\0'),
                    None => break,
                }
            }
        }

        let positions = self.offset_col_nums().map(|n| self.col_num_to_col_idx(n));

        // Go through the column numbers and their positions
        // col is the actual index in the column in the 2D array
        'outer: for col in positions {
            for row in 0..HEIGHT {
                if stencil[(row, col)] == Symbol::Filled('\0') {
                    match symbols.next() {
                        Some(c) => stencil[(row, col)] = Symbol::Filled(c),
                        None => break 'outer,
                    }
                }
            }
        }

        // Read off the characters starting from the correct point
        let mut output = String::new();
        for idx in self.wrapping_iter(start) {
            if stencil[idx].is_filled() {
                output.push(stencil[idx].to_char())
            }
        }

        Ok(output)
    }
}

#[cfg(test)]
mod rs44_tests {

    use super::*;

    // check configuration for default
    // https://derekbruff.org/blogs/fywscrypto/historical-crypto/rasterschlussel-44-the-stencil-on-steroids/
    const PLAINTEXT: &'static str =
        "RAINNBOWUNICORNHORNSAREIMMENSELYMOREVALUABLETHANTHOSEOFEVENTHELARGESTNARWHALS";
    const CIPHERTEXT: &'static str =
        "HNANOESONMEGNANAALHRNTRAUHVSCWSTNAOAWVIBHMEFLREMLRNRLTIOEAEEBRSUIYEHREOTOLSEN";
    const STENCIL: &'static str = "⬛⬜⬛⬛⬛⬛⬛⬜⬜⬛⬛⬜⬛⬜⬛⬜⬛⬛⬜⬜⬛⬛⬜⬜⬛⬛⬛⬜⬛⬜⬛⬛⬜⬛⬜⬜⬛⬛⬜⬛⬛⬛⬜⬛⬛⬜⬛⬜⬛⬜⬜⬛⬛⬜⬜⬜⬛⬛⬛⬛⬜⬛⬛⬜⬛⬜⬛⬜⬛⬜⬛⬛⬛⬜⬛⬛⬜⬛⬜⬛⬜⬜⬜⬛⬜⬛⬛⬜⬛⬛⬜⬛⬛⬜⬛⬛⬜⬛⬛⬛⬜⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬜⬛⬜⬜⬛⬜⬛⬛⬛⬛⬜⬜⬛⬜⬛⬛⬛⬜⬛⬛⬜⬛⬛⬜⬛⬜⬜⬜⬛⬛⬜⬜⬛⬛⬜⬛⬛⬜⬛⬜⬛⬛⬛⬜⬛⬛⬛⬛⬜⬛⬜⬛⬛⬜⬛⬛⬛⬜⬜⬜⬛⬛⬜⬜⬛⬛⬜⬛⬜⬛⬜⬛⬛⬜⬜⬛⬛⬛⬜⬜⬛⬛⬜⬛⬛⬜⬛⬜⬛⬛⬜⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬜⬜⬛⬛⬜⬛⬜⬜⬛⬛⬜⬜⬜⬛⬜⬜⬛⬜⬛⬜⬛⬛⬜⬜⬛⬜⬛⬛⬛⬜⬜⬛⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬜⬛⬛⬜⬜⬜⬛⬜⬛⬛⬜⬛⬛⬜⬜⬛⬛⬛⬜⬛⬜⬛⬜⬛⬜⬜⬛⬜⬛⬜⬛⬜⬛⬛⬛⬜⬛⬜⬛⬛⬜⬛⬜⬛⬛⬛⬜⬛⬛⬜⬛⬛⬜⬜⬛⬛⬜⬛RA⬛⬛⬛I⬛⬛NNB⬛OWU⬛⬛⬛⬛N⬛⬛⬛⬛I⬛⬛⬛CO⬛R⬛⬛⬛⬛⬛N⬛⬛⬛HO⬛R⬛⬛NS⬛⬛AR⬛⬛⬛EI⬛⬛⬛MM⬛⬛EN⬛⬛⬛S⬛⬛⬛⬛E⬛⬛⬛LYMO⬛⬛RE⬛V⬛⬛A⬛⬛LU⬛A⬛B⬛L⬛⬛⬛⬛⬛E⬛T⬛⬛⬛⬛H⬛⬛A⬛⬛⬛N⬛THO⬛⬛⬛SE⬛⬛O⬛⬛FE⬛V⬛⬛⬛E⬛⬛⬛⬛N⬛TH⬛⬛E⬛LA⬛R⬛⬛G⬛⬛E⬛⬛ST⬛⬛NA⬛⬛R⬛W⬛H⬛⬛⬛A⬛L⬛⬛S⬛⬛⬜⬜⬛⬛⬛⬛⬜⬜⬛⬛⬜⬛⬜⬛⬜⬛⬛⬛⬛⬜⬛⬜⬛⬜⬛⬛⬛⬜⬜⬛⬛⬛⬛⬜⬛⬛⬜⬜⬛⬜⬛⬜⬛⬜⬛⬛⬛⬛⬜⬜⬛⬜⬛⬛⬜⬛⬜⬜⬛⬛⬜⬛⬛⬜⬛⬜⬛⬛⬛⬛⬜⬛⬛⬛⬛⬜⬜⬛⬛⬜⬜⬜⬛⬛⬜⬜⬜⬛⬛⬛⬜⬛";

    // We know the stencil is filled in correctly based on a reference image
    #[test]
    fn grid_test() {
        let mut cipher = Rs44::default();

        let mut symbols = PLAINTEXT.chars();
        let start = cipher.stencil.index_from_coord((12, 16)).unwrap();
        for idx in start..600 {
            if cipher.stencil[idx].is_empty() {
                match symbols.next() {
                    Some(c) => cipher.stencil[idx] = Symbol::Filled(c),
                    None => break,
                }
            }
        }
        assert_eq!(
            format!("{}", cipher.stencil)
                .split_whitespace()
                .collect::<String>(),
            STENCIL
        );
    }

    #[test]
    fn encrypt_test() {
        let mut cipher = Rs44::default();
        cipher.start_cell = (12, 16);
        cipher.start_column = 7;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Rs44::default();
        cipher.start_cell = (12, 16);
        cipher.start_column = 7;
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }

    #[test]
    fn wrap_test() {
        let mut cipher = Rs44::default();
        cipher.start_cell = (20, 20);
        cipher.start_column = 7;
        let ciphertext = cipher.encrypt(PLAINTEXT).unwrap();
        assert_eq!(cipher.decrypt(&ciphertext).unwrap(), PLAINTEXT);
    }
}
