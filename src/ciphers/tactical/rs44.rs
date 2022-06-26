use itertools::Itertools;
use rand::{
    prelude::{SliceRandom, StdRng},
    Rng, SeedableRng,
};

const DEFAULT_STENCIL: &'static str = "⬛⬜⬛⬛⬛⬛⬛⬜⬜⬛⬛⬜⬛⬜⬛⬜⬛⬛⬜⬜⬛⬛⬜⬜⬛⬛⬛⬜⬛⬜⬛⬛⬜⬛⬜⬜⬛⬛⬜⬛⬛⬛⬜⬛⬛⬜⬛⬜⬛⬜⬜⬛⬛⬜⬜⬜⬛⬛⬛⬛⬜⬛⬛⬜⬛⬜⬛⬜⬛⬜⬛⬛⬛⬜⬛⬛⬜⬛⬜⬛⬜⬜⬜⬛⬜⬛⬛⬜⬛⬛⬜⬛⬛⬜⬛⬛⬜⬛⬛⬛⬜⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬜⬛⬜⬜⬛⬜⬛⬛⬛⬛⬜⬜⬛⬜⬛⬛⬛⬜⬛⬛⬜⬛⬛⬜⬛⬜⬜⬜⬛⬛⬜⬜⬛⬛⬜⬛⬛⬜⬛⬜⬛⬛⬛⬜⬛⬛⬛⬛⬜⬛⬜⬛⬛⬜⬛⬛⬛⬜⬜⬜⬛⬛⬜⬜⬛⬛⬜⬛⬜⬛⬜⬛⬛⬜⬜⬛⬛⬛⬜⬜⬛⬛⬜⬛⬛⬜⬛⬜⬛⬛⬜⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬜⬜⬛⬛⬜⬛⬜⬜⬛⬛⬜⬜⬜⬛⬜⬜⬛⬜⬛⬜⬛⬛⬜⬜⬛⬜⬛⬛⬛⬜⬜⬛⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬜⬛⬛⬜⬜⬜⬛⬜⬛⬛⬜⬛⬛⬜⬜⬛⬛⬛⬜⬛⬜⬛⬜⬛⬜⬜⬛⬜⬛⬜⬛⬜⬛⬛⬛⬜⬛⬜⬛⬛⬜⬛⬜⬛⬛⬛⬜⬛⬛⬜⬛⬛⬜⬜⬛⬛⬜⬛⬜⬜⬛⬛⬛⬜⬛⬛⬜⬜⬜⬛⬜⬜⬜⬛⬛⬛⬛⬜⬛⬛⬛⬛⬜⬛⬛⬛⬜⬜⬛⬜⬛⬛⬛⬛⬛⬜⬛⬛⬛⬜⬜⬛⬜⬛⬛⬜⬜⬛⬛⬜⬜⬛⬛⬛⬜⬜⬛⬛⬛⬜⬜⬛⬛⬜⬜⬛⬛⬛⬜⬛⬛⬛⬛⬜⬛⬛⬛⬜⬜⬜⬜⬛⬛⬜⬜⬛⬜⬛⬛⬜⬛⬛⬜⬜⬛⬜⬛⬜⬛⬜⬛⬛⬛⬛⬛⬜⬛⬜⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬛⬜⬛⬜⬜⬜⬛⬛⬛⬜⬜⬛⬛⬜⬛⬛⬜⬜⬛⬜⬛⬛⬛⬜⬛⬛⬛⬛⬜⬛⬜⬜⬛⬛⬜⬛⬜⬜⬛⬜⬛⬛⬜⬛⬛⬜⬛⬛⬜⬜⬛⬛⬜⬜⬛⬛⬜⬛⬜⬛⬜⬛⬛⬛⬜⬛⬜⬛⬛⬜⬛⬛⬜⬜⬛⬛⬛⬛⬜⬜⬛⬛⬜⬛⬜⬛⬜⬛⬛⬛⬛⬜⬛⬜⬛⬜⬛⬛⬛⬜⬜⬛⬛⬛⬛⬜⬛⬛⬜⬜⬛⬜⬛⬜⬛⬜⬛⬛⬛⬛⬜⬜⬛⬜⬛⬛⬜⬛⬜⬜⬛⬛⬜⬛⬛⬜⬛⬜⬛⬛⬛⬛⬜⬛⬛⬛⬛⬜⬜⬛⬛⬜⬜⬜⬛⬛⬜⬜⬜⬛⬛⬛⬜⬛";

use crate::{
    ciphers::Cipher,
    errors::CipherError,
    global_rng::get_global_rng,
    grid::{Grid, Symbol, EMPTY, BLOCK},
    text_aux::{PresetAlphabet},
};

pub struct RS44 {
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

impl Default for RS44 {
    fn default() -> Self {
        let mut rng = StdRng::seed_from_u64(5920348976);

        // Build the stencile
        let mut stencil: Grid<Symbol<char>> = Grid::new_blocked(Self::HEIGHT, Self::WIDTH);
        for (i, c) in DEFAULT_STENCIL.chars().enumerate() {
            if c == EMPTY {
                stencil[i] = Symbol::Empty;
            }
        }
        let column_nums = [
            13, 1, 20, 10, 18, 14, 0, 7, 17, 9, 23, 2, 6, 11, 16, 19, 4, 12, 22, 15, 5, 3, 21, 24, 8,
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

    pub fn encrypt_label_char(&self, c: char, rng: &mut StdRng) -> char {
        let row: usize = rng.gen_range(0..5);
        let col = self.label_letter_to_matrix_column(c);
        self.message_key_maxtrix[(row, col)]
    }

    pub fn set_full_message_key(&mut self) {
        self.encrypted_message_key.clear();
        let mut rng = get_global_rng();
        self.xlabels[self.start_cell.1].chars().for_each(|c| self.encrypted_message_key.push(self.encrypt_label_char(c, &mut rng)));
        self.ylabels[self.start_cell.0].chars().for_each(|c| self.encrypted_message_key.push(self.encrypt_label_char(c, &mut rng)));
        self.encrypted_message_key.push('-');
        self.xlabels[self.start_column].chars().for_each(|c| self.encrypted_message_key.push(self.encrypt_label_char(c, &mut rng)));
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

    pub fn stencil_to_text(&self) -> String {
        self.stencil.get_rows().map(|c| c.to_char()).collect()
    }

    pub fn text_to_stencil(&mut self) -> Result<(),CipherError> {
        let mut vec = Vec::with_capacity(Self::GRID_SIZE);
        let mut ctr = 0;
        for (n,c) in self.imported_stencil.chars().enumerate() {
            if c == EMPTY {
                vec.push(Symbol::Empty);
                ctr += 1;
            } else if c == BLOCK {
                vec.push(Symbol::Empty)
            } else {
                return Err(CipherError::Key(format!("The RS44 key can only be built from the symbols {} and {}", EMPTY, BLOCK)))
            }
            if (n+1) % 25 == 0 && ctr % 10 != 0{
                return Err(CipherError::key("The RS44 stencil must have exactly 10 empty spaces in each row"))
            }
        }
        if vec.len() != Self::GRID_SIZE {
            return Err(CipherError::key("The RS44 key must have exactly 600 positions defined"))
        }
        self.stencil = Grid::from_rows(vec, 24, 25);
        self.imported_stencil.clear();
        Ok(())
    }
}

impl Cipher for RS44 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        match self.stencil.get(self.start_cell) {
            Some(s) => {
                if !s.is_empty() {
                    return Err(CipherError::key(
                        "starting cell must be an empty position",
                    ));
                } else {
                    ()
                }
            }
            None => return Err(CipherError::key("starting cell out of bounds")),
        }
        if self.start_column > 24 {
            return Err(CipherError::key("starting column out of bounds"))
        }

        let mut output = String::with_capacity(text.len());

        let mut symbols = text.chars();
        let mut stencil = self.stencil.clone();
        let start = stencil.index_from_coord(self.start_cell).unwrap();

        for idx in start..Self::GRID_SIZE {
            if stencil[idx].is_empty() {
                match symbols.next() {
                    Some(c) => stencil[idx] = Symbol::Character(c),
                    None => { break }
                }
            }
        }

        let start_idx = self.column_nums[self.start_column];
        let positions = (start_idx..25).chain(0..start_idx).map(|n| self.column_nums.iter().position(|x| &n == x).unwrap());

        for col in positions {
            let s: String = stencil
                .get_col(col)
                .filter(|sym| sym.is_character())
                .map(|sym| sym.to_char())
                .collect();
            //dbg!(&s);
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
                .position(|n| *n == pos)
                .unwrap();
            // The starting y-value is one less than the message key y-value until we reach it
            let y_min = match pos < self.start_cell.1 {
                true => self.start_cell.0 - 1,
                false => self.start_cell.0,
            };
            for y in y_min..Self::HEIGHT {
                if stencil[(x, y)].is_empty() {
                    match symbols.next() {
                        Some(c) => stencil[(x, y)] = Symbol::Character(c),
                        None => { break }
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
        self.randomize_stencil();
        self.randomize_matrix();
        self.randomize_labels();
        self.set_full_message_key();
    }
}


#[cfg(test)]
mod rs44_tests {

    use super::*;
 
    // check configuration for default
    // https://derekbruff.org/blogs/fywscrypto/historical-crypto/rasterschlussel-44-the-stencil-on-steroids/
    const PLAINTEXT: &'static str = "RAINNBOWUNICORNHORNSAREIMMENSELYMOREVALUABLETHANTHOSEOFEVENTHELARGESTNARWHALS";
    const CIPHERTEXT: &'static str = "HNANOESONMEGNANAALHRNTRAUHVSCWSTNAOAWVIBHMEFLREMLRNRLTIOEAEEBRSUIYEHREOTOLSEN";
    const STENCIL: &'static str = "⬛⬜⬛⬛⬛⬛⬛⬜⬜⬛⬛⬜⬛⬜⬛⬜⬛⬛⬜⬜⬛⬛⬜⬜⬛⬛⬛⬜⬛⬜⬛⬛⬜⬛⬜⬜⬛⬛⬜⬛⬛⬛⬜⬛⬛⬜⬛⬜⬛⬜⬜⬛⬛⬜⬜⬜⬛⬛⬛⬛⬜⬛⬛⬜⬛⬜⬛⬜⬛⬜⬛⬛⬛⬜⬛⬛⬜⬛⬜⬛⬜⬜⬜⬛⬜⬛⬛⬜⬛⬛⬜⬛⬛⬜⬛⬛⬜⬛⬛⬛⬜⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬜⬛⬜⬜⬛⬜⬛⬛⬛⬛⬜⬜⬛⬜⬛⬛⬛⬜⬛⬛⬜⬛⬛⬜⬛⬜⬜⬜⬛⬛⬜⬜⬛⬛⬜⬛⬛⬜⬛⬜⬛⬛⬛⬜⬛⬛⬛⬛⬜⬛⬜⬛⬛⬜⬛⬛⬛⬜⬜⬜⬛⬛⬜⬜⬛⬛⬜⬛⬜⬛⬜⬛⬛⬜⬜⬛⬛⬛⬜⬜⬛⬛⬜⬛⬛⬜⬛⬜⬛⬛⬜⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬜⬜⬛⬛⬜⬛⬜⬜⬛⬛⬜⬜⬜⬛⬜⬜⬛⬜⬛⬜⬛⬛⬜⬜⬛⬜⬛⬛⬛⬜⬜⬛⬛⬜⬛⬛⬛⬛⬛⬛⬜⬛⬛⬜⬛⬛⬜⬛⬛⬜⬜⬜⬛⬜⬛⬛⬜⬛⬛⬜⬜⬛⬛⬛⬜⬛⬜⬛⬜⬛⬜⬜⬛⬜⬛⬜⬛⬜⬛⬛⬛⬜⬛⬜⬛⬛⬜⬛⬜⬛⬛⬛⬜⬛⬛⬜⬛⬛⬜⬜⬛⬛⬜⬛RA⬛⬛⬛I⬛⬛NNB⬛OWU⬛⬛⬛⬛N⬛⬛⬛⬛I⬛⬛⬛CO⬛R⬛⬛⬛⬛⬛N⬛⬛⬛HO⬛R⬛⬛NS⬛⬛AR⬛⬛⬛EI⬛⬛⬛MM⬛⬛EN⬛⬛⬛S⬛⬛⬛⬛E⬛⬛⬛LYMO⬛⬛RE⬛V⬛⬛A⬛⬛LU⬛A⬛B⬛L⬛⬛⬛⬛⬛E⬛T⬛⬛⬛⬛H⬛⬛A⬛⬛⬛N⬛THO⬛⬛⬛SE⬛⬛O⬛⬛FE⬛V⬛⬛⬛E⬛⬛⬛⬛N⬛TH⬛⬛E⬛LA⬛R⬛⬛G⬛⬛E⬛⬛ST⬛⬛NA⬛⬛R⬛W⬛H⬛⬛⬛A⬛L⬛⬛S⬛⬛⬜⬜⬛⬛⬛⬛⬜⬜⬛⬛⬜⬛⬜⬛⬜⬛⬛⬛⬛⬜⬛⬜⬛⬜⬛⬛⬛⬜⬜⬛⬛⬛⬛⬜⬛⬛⬜⬜⬛⬜⬛⬜⬛⬜⬛⬛⬛⬛⬜⬜⬛⬜⬛⬛⬜⬛⬜⬜⬛⬛⬜⬛⬛⬜⬛⬜⬛⬛⬛⬛⬜⬛⬛⬛⬛⬜⬜⬛⬛⬜⬜⬜⬛⬛⬜⬜⬜⬛⬛⬛⬜⬛";

    // We know the stencil is filled in correctly based on a reference image
    #[test]
    fn grid_test() {
        let mut cipher = RS44::default();

        let mut symbols = PLAINTEXT.chars();
        let start = cipher.stencil.index_from_coord((12,16)).unwrap();
        for idx in start..600 {
            if cipher.stencil[idx].is_empty() {
                match symbols.next() {
                    Some(c) => cipher.stencil[idx] = Symbol::Character(c),
                    None => { break }
                }
            }
        }
        assert_eq!(format!("{}",cipher.stencil).split_whitespace().collect::<String>(),STENCIL);
    }

    #[test]
    fn encrypt_test() {
        let mut cipher = RS44::default();
        cipher.start_cell = (12,16);
        cipher.start_column = 7;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = RS44::default();
        cipher.start_cell = (12,16);
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}