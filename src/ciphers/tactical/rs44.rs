use itertools::Itertools;
use rand::{prelude::{StdRng, SliceRandom}, SeedableRng, Rng};

use crate::{grid::{Symbol, Grid}, errors::CipherError, ciphers::Cipher, global_rng::get_gobal_rng, text_aux::PresetAlphabet};

pub struct RS44 {
    stencil: Grid<Symbol<char>>,
    column_nums: [u8; 25],
    xlabels: [&'static str; 25],
    ylabels: [&'static str; 24],
    message_key: (usize,usize),
    message_key_maxtrix: Grid<char>,
    time: String,
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
                stencil[n+(i*Self::WIDTH)] = Symbol::Empty;
            }
        }
        let column_nums = [
            24,  0, 19, 12, 15, 
            3 , 13,  6,  4, 21, 
            11, 17, 22,  5,  1, 
             9, 10, 18, 23, 16,
             2,  7, 14,  8, 20
        ];
        let xlabels: [&str; Self::WIDTH] = {
            let mut arr = Self::LABELS.clone();
            arr.shuffle(&mut rng);
            arr
        };
        let ylabels: [&str; Self::HEIGHT] = {
            let mut arr: [&str; Self::HEIGHT] = Self::LABELS.clone().iter().map(|x| *x).take(Self::HEIGHT).collect_vec().try_into().unwrap();
            arr.shuffle(&mut rng);
            arr
        };
        let message_key_maxtrix = {
            let mut g: Grid<char> = Grid::from_rows(
                PresetAlphabet::BasicLatinNoJ.chars().map(|c| c.to_ascii_lowercase()).collect_vec(), 
                5, 5);
            g.shuffle(&mut rng);
            g
        };

        Self { stencil, column_nums, xlabels, ylabels, message_key: (0,0), message_key_maxtrix, time: String::new() }
        
    }
}

impl RS44 {
    pub const WIDTH: usize = 25;
    pub const HEIGHT: usize = 24;
    pub const LABELS: [&'static str; 25] = 
        ["aa", "ba", "ca", "da", "ea",
         "ab", "bb", "cb", "db", "eb",
         "ac", "bc", "cc", "dc", "ec",
         "ad", "bd", "cd", "dd", "de",
         "ae", "be", "ce", "de", "ee"];
    pub const MESSAGE_LENGTH: usize = 240;
    pub const GRID_SIZE: usize = 600;
    
    fn label_letter_to_matrix_column(&self, c: char) -> usize {
        match c {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            _ => unreachable!("the only letter used for labels are 'abcde'")
        }
    }
    
    pub fn encrypt_message_key(&self) -> Result<String,CipherError> {
        match self.stencil.get(self.message_key) {
            Some(s) => if !s.is_empty() { return Err(CipherError::key("message key must select an empty position")) } else { () }
            None => return Err(CipherError::key("message key out of bounds"))
        }
        let mut message_key_string = String::with_capacity(4);
        let mut rng = get_gobal_rng();
        for c in self.xlabels[self.message_key.0].chars().chain(self.ylabels[self.message_key.1].chars()) {
            let row: usize = rng.gen_range(0..5);
            let col = self.label_letter_to_matrix_column(c);
            message_key_string.push(self.message_key_maxtrix[(row,col)]);
        }
        
        Ok(message_key_string)
    }
    
    pub fn decrypt_message_key(&self) -> Result<(usize,usize),CipherError> {
        todo!("decrypt the message key")
    }
    
    pub fn randomize_stencil(&mut self) {
        self.stencil.apply(|x| Symbol::Blocked);
        let mut rng = get_gobal_rng();
        let mut positions: Vec<usize> = (0..Self::WIDTH).collect();
        
        for i in 0..Self::HEIGHT {
            positions.shuffle(&mut *rng);
            for n in &positions[0..10] {
                self.stencil[n+(i*Self::WIDTH)] = Symbol::Empty;
            }
        }
    }
    
    // Start at the given position and give positions going down columns, wrapping around
    // This is only called after the message key is checked the start position is always valid
    fn vec_positions(start: (usize,usize)) -> Vec<(usize,usize)> {
        let mut positions = Vec::with_capacity(Self::GRID_SIZE);
        let mut current = start;
        for i in 0..Self::GRID_SIZE {
            positions.push(current);
            current.1 = (current.1 + 1) % 24;
            if current.1 == 0 {
                current.0 = (current.0 + 1) % 25
            }
        }
        positions
    }
}



impl Cipher for RS44 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut output = self.encrypt_message_key()?;
        output.push_str(&self.time);
        output.push_str(&format!("{}",text.chars().count()));

        

        todo!("
        steps for encyrption:
            encrypt the message key, also validating at the same time
            prepend the encrypted key, the time, and the length
            clone the stencil
            iterate through the stencil by rows, filling in the empty spaces
            give an error if we run out of space
            read the stencil off by columns in the order given by column_nums
        ")
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!("
        steps for decyrption:
            decrypt the message key, also validating at the same time
            check that the length is correct
            clone the stencil
            write into the stencil by column in the order given by column_nums using the message key as a guide
            give an error if we run out of space
            read the stencil off by rows
        ")
    }

    fn reset(&mut self) {
        *self = Self::default();
    }

    fn randomize(&mut self) {
        todo!("randomize stencil and maxtrix");
    }
}