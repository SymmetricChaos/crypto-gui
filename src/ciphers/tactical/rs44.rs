use rand::{prelude::{StdRng, SliceRandom}, SeedableRng, Rng};

use crate::{grid::{Symbol, Grid}, errors::CipherError, ciphers::Cipher, global_rng::get_gobal_rng};

pub struct RS44 {
    stencil: Grid<Symbol<char>>,
    xlabels: [&'static str; 25],
    ylabels: [&'static str; 24],
    message_key: (usize,usize),
    message_key_maxtrix: Grid<char>,
    _time: String,
    seed: Option<u64>,
}

impl Default for RS44 {
    fn default() -> Self {
        todo!("build from a seed value")
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
    
    fn get_rng(&self) -> StdRng {
        match self.seed {
            Some(n) => SeedableRng::seed_from_u64(n),
            None => SeedableRng::from_entropy(),
        }
    }
    
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
        let mut rng = self.get_rng();
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
        self.stencil.apply(|_| Symbol::Blocked);
        let mut rng = self.get_rng();
        let mut positions: Vec<usize> = (0..Self::WIDTH).collect();
        
        for i in 0..Self::HEIGHT {
            positions.shuffle(&mut rng);
            for n in &positions[0..10] {
                self.stencil[n+(i*Self::WIDTH)] = Symbol::Empty;
            }
        }
    }
    
    // Start at the given position and give positions going down columns, wrapping around
    // This is only called after the message key is checked the start position is always valid
    fn open_positions(&self, start: (usize,usize)) -> Vec<(usize,usize)> {
        let mut positions = Vec::with_capacity(Self::MESSAGE_LENGTH);
        let mut current = start;
        for _ in 0..Self::GRID_SIZE {
            if self.stencil[current].is_empty() {
                positions.push(current);
            }
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
        let mut out = String::with_capacity(Self::MESSAGE_LENGTH+5);
        out.push_str(&self.encrypt_message_key()?);
        out.push(' ');
        let positions = self.open_positions(self.message_key);
        let mut temp_stencil = self.stencil.clone();
        let mut chars = text.chars();
        for pos in positions.iter() {
            temp_stencil[*pos] = Symbol::Character(chars.next().expect("need to all nulls here"));
        }
        todo!("read the text out and append to the output")
    }

    fn decrypt(&self, _text: &str) -> Result<String, CipherError> {
        let _out = String::new();
        let _positions = self.open_positions(self.message_key);
        
        todo!("decrypt it lol")
    }

    fn reset(&mut self) {
        *self = Self::default();
    }

    fn randomize(&mut self) {
        let mut _rng = &mut get_gobal_rng();
        todo!("randomize stencil and matrix");
    }
}