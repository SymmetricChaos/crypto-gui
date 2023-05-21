use num::Integer;


use crate::text_aux::VecString;
use crate::{errors::CipherError, ciphers::Cipher};
use crate::grid::{Grid, Symbol};

pub enum Parity {
    Odd,
    Even
}

pub struct Amsco {
    alphabet_string: String,
    alphabet: VecString,
    key: Vec<usize>,
    pub key_word: String,
    parity: Parity
}

impl Default for Amsco {
    fn default() -> Self {
        Self {
            alphabet_string: String::from(PresetAlphabet::BasicLatin),
            alphabet: VecString::from(BasicLatin),
            key: Vec::new(),
            key_word: String::new(),
            parity: Parity::Odd,
        }
    }
}

impl Amsco {

    pub fn set_alphabet(&mut self) {
        self.alphabet = VecString::unique_from(&self.alphabet_string);
    }

    pub fn assign_alphabet(&mut self, alphabet: &str) {
        self.alphabet_string = String::from(alphabet);
        self.set_alphabet();
    }
    
    pub fn set_key(&mut self) {
        self.key = rank_str(&self.key_word, &self.alphabet);
    }

    pub fn assign_key(&mut self, key_word: &str) {
        self.key_word = key_word.to_string();
        self.key = rank_str(&self.key_word, &self.alphabet);
    }
    
    pub fn pattern(&self) {
        let pattern = match self.parity {
            Parity::Odd => [1,2],
            Parity::Even => [2,1],
        };
        pattern.iter().cycle()
    }
}

impl Cipher for Amsco {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let tlen = text.chars().count();
        let n_cols = self.key.len();
        if n_cols < 2 {
            return Err(CipherError::key(
                "The key for an AMSCO cipher must have at least two characters",
            ));
        }



        
        let symbols = str_to_char_grid(text, '\0', '\0');
        let g = Grid::<&str>::from_cols(symbols, n_rows, n_cols);


        let mut out = String::with_capacity(text.len());
        for k in self.key.iter() {
            let mut s: String = g.get_col(*k).map(|sym| sym.to_char()).collect();
            s = s.replace(crate::grid::EMPTY, "");
            s = s.replace(crate::grid::BLOCK, "");
            out.push_str(&s);
        }

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let tlen = text.chars().count();
        let n_cols = self.key.len();
        if n_cols < 2 {
            return Err(CipherError::key(
                "The key for an AMSCO cipher must have at least two characters",
            ));
        }
        todo!()
    }

    fn randomize(&mut self) {
        self.assign_key(&self.alphabet.get_rand_chars_replace(11, rng).collect());
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}