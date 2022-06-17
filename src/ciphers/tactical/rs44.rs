use itertools::Itertools;
use rand::{
    prelude::{SliceRandom, StdRng},
    Rng, SeedableRng,
};

use crate::{
    ciphers::Cipher,
    errors::CipherError,
    global_rng::get_global_rng,
    grid::{Grid, Symbol},
    text_aux::PresetAlphabet,
};

pub struct RS44 {
    stencil: Grid<Symbol<char>>,
    column_nums: [u8; 25],
    xlabels: [&'static str; 25],
    ylabels: [&'static str; 24],
    message_key: (usize, usize),
    message_key_maxtrix: Grid<char>,
    hours: u8,
    minutes: u8,
}

impl Default for RS44 {
    fn default() -> Self {
        let mut rng = StdRng::seed_from_u64(3141592654);

        // Build the stencile
        let mut stencil: Grid<Symbol<char>> = Grid::new_empty(Self::WIDTH, Self::HEIGHT);
        let mut positions: Vec<usize> = (0..Self::WIDTH).collect();
        for i in 0..Self::HEIGHT {
            positions.shuffle(&mut rng);
            for n in &positions[0..10] {
                stencil[n + (i * Self::WIDTH)] = Symbol::Empty;
            }
        }
        let column_nums = [
            24, 0, 19, 12, 15, 3, 13, 6, 4, 21, 11, 17, 22, 5, 1, 9, 10, 18, 23, 16, 2, 7, 14, 8,
            20,
        ];
        let xlabels: [&str; Self::WIDTH] = {
            let mut arr = Self::LABELS.clone();
            arr.shuffle(&mut rng);
            arr
        };
        let ylabels: [&str; Self::HEIGHT] = {
            let mut v = Self::LABELS.clone();
            v.shuffle(&mut rng);
            v.iter()
                .take(Self::HEIGHT)
                .map(|x| *x)
                .collect_vec()
                .try_into()
                .unwrap()
        };
        let message_key_maxtrix = {
            let mut g: Grid<char> = Grid::from_rows(
                PresetAlphabet::BasicLatinNoJ
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
            message_key: (0, 0),
            message_key_maxtrix,
            hours: 0,
            minutes: 0,
        }
    }
}

impl RS44 {
    pub const WIDTH: usize = 25;
    pub const HEIGHT: usize = 24;
    pub const LABELS: [&'static str; 25] = [
        "aa", "ba", "ca", "da", "ea", "ab", "bb", "cb", "db", "eb", "ac", "bc", "cc", "dc", "ec",
        "ad", "bd", "cd", "dd", "de", "ae", "be", "ce", "de", "ee",
    ];
    pub const MESSAGE_LENGTH: usize = 240;
    pub const GRID_SIZE: usize = 600;

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

    pub fn encrypt_message_key(&self) -> Result<String, CipherError> {
        match self.stencil.get(self.message_key) {
            Some(s) => {
                if !s.is_empty() {
                    return Err(CipherError::key(
                        "message key must select an empty position",
                    ));
                } else {
                    ()
                }
            }
            None => return Err(CipherError::key("message key out of bounds")),
        }
        let mut message_key_string = String::with_capacity(4);
        let mut rng = get_global_rng();
        for c in self.xlabels[self.message_key.0]
            .chars()
            .chain(self.ylabels[self.message_key.1].chars())
        {
            let row: usize = rng.gen_range(0..5);
            let col = self.label_letter_to_matrix_column(c);
            message_key_string.push(self.message_key_maxtrix[(row, col)]);
        }

        Ok(message_key_string)
    }

    pub fn full_message_key(&self) -> Result<String, CipherError> {
        let mut output = String::with_capacity(13);
        output.push_str(&self.encrypt_message_key()?);
        output.push_str(&format!("-{:02}{:02}", self.hours, self.minutes));
        Ok(output)
    }

    pub fn randomize_stencil(&mut self) {
        self.stencil.apply(|_| Symbol::Blocked);
        let mut rng = get_global_rng();
        let mut positions: Vec<usize> = (0..Self::WIDTH).collect();

        for i in 0..Self::HEIGHT {
            positions.shuffle(&mut *rng);
            for n in &positions[0..10] {
                self.stencil[n + (i * Self::WIDTH)] = Symbol::Empty;
            }
        }
    }

    pub fn randomize_matrix(&mut self) {
        self.message_key_maxtrix.shuffle(&mut *get_global_rng())
    }

    pub fn randomize_labels(&mut self) {
        let mut rng = get_global_rng();
        self.column_nums.shuffle(&mut *rng);
        self.xlabels.shuffle(&mut *rng);
        self.ylabels = {
            let mut v = Self::LABELS.clone();
            v.shuffle(&mut *rng);
            v.iter()
                .take(Self::HEIGHT)
                .map(|x| *x)
                .collect_vec()
                .try_into()
                .unwrap()
        };
    }
}

impl Cipher for RS44 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        match self.stencil.get(self.message_key) {
            Some(s) => {
                if !s.is_empty() {
                    return Err(CipherError::key(
                        "message key must select an empty position",
                    ));
                } else {
                    ()
                }
            }
            None => return Err(CipherError::key("message key out of bounds")),
        }

        let mut output = String::with_capacity(text.len());

        let mut symbols = text.chars();
        let mut stencil = self.stencil.clone();
        let start = stencil.index_from_coord(self.message_key).unwrap();

        for idx in start..Self::GRID_SIZE {
            if stencil[idx].is_empty() {
                match symbols.next() {
                    Some(c) => stencil[idx] = Symbol::Character(c),
                    None => {
                        return Err(CipherError::input(
                            "ran out of spaces before finishing message",
                        ))
                    }
                }
            }
        }

        for k in self.column_nums.iter() {
            let s: String = stencil
                .get_col(*k as usize)
                .filter(|sym| sym.is_character())
                .map(|sym| sym.to_char())
                .collect();
            output.push_str(&s);
        }

        Ok(output)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut symbols = text.chars();
        let mut stencil = self.stencil.clone();

        for pos in 0..Self::WIDTH {
            // unwrap justified by static size
            let x = self
                .column_nums
                .iter()
                .position(|n| *n == pos as u8)
                .unwrap();
            // The starting y-value is one less than the message key y-value until we reach it
            let y_min = match pos < self.message_key.1 {
                true => self.message_key.0 - 1,
                false => self.message_key.0,
            };
            for y in y_min..Self::HEIGHT {
                if stencil[(x, y)].is_empty() {
                    match symbols.next() {
                        Some(c) => stencil[(x, y)] = Symbol::Character(c),
                        None => {
                            return Err(CipherError::input(
                                "ran out of spaces before finishing message",
                            ))
                        }
                    }
                }
            }
        }

        Ok(stencil.read_rows_characters().collect())
    }

    fn reset(&mut self) {
        *self = Self::default();
    }

    fn randomize(&mut self) {
        todo!("randomize stencil and maxtrix");
    }
}
