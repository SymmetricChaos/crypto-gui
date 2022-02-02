use std::fmt;

use num::integer::Roots;
use rand::prelude::ThreadRng;
use super::Cipher;
use crate::{errors::CipherError, text_functions::shuffled_str};

use crate::text_functions::{LATIN_UPPER_NO_J, LATIN_UPPER_NO_Q, LATIN_UPPER_DIGITS, validate_alphabet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayfairMode {
    NoQ,
    NoJ,
    AlphaNum,
}

pub struct Playfair {
    alphabet: String,
    spacer: char,
}

impl Playfair {

    pub fn set_mode(&mut self, mode: PlayfairMode) {
        match mode {
            PlayfairMode::NoQ => self.alphabet = String::from(LATIN_UPPER_NO_Q),
            PlayfairMode::NoJ => self.alphabet = String::from(LATIN_UPPER_NO_J),
            PlayfairMode::AlphaNum => self.alphabet = String::from(LATIN_UPPER_DIGITS),
        };
    }

    fn validate_settings(&self) -> Result<(),CipherError> {
        
        validate_alphabet(&self.alphabet)?;
        if !&self.alphabet.contains(self.spacer) {
            return Err(CipherError::Key(format!("spacer character {} is not in the alphabet",self.spacer)))
        }
        if self.size() != self.grid_size().pow(2) {
            return Err(CipherError::Alphabet(String::from("alphabet length must be a square number in order to exactly fill the grid")))
        }
        Ok(())
    }

    fn pairs(&self, text: &str) -> Result<Vec<(char,char)>,CipherError> {
        let mut symbols: Vec<char> = text.chars().rev().collect();
        let mut out = Vec::with_capacity(text.len()/2);
        while symbols.len() >= 2 {
            //unwrap justified by condition above
            let l = symbols.pop().unwrap();
            let r = symbols.pop().unwrap();
            if l == r {
                symbols.push(r);
                out.push((l,self.spacer));
            } else {
                out.push((l,r));
            }
        }
        match symbols.len() == 0 {
            true => Ok(out),
            false => Err(CipherError::input("Input text does not divide into pairs. Input text may have an odd number of characters or a double letter may have changed the number of characters.")),
        }
    }

    pub fn size(&self) -> usize {
        self.alphabet.chars().count()
    }

    pub fn grid_size(&self) -> usize {
        self.size().sqrt()
    }

    fn char_to_position(&self,symbol: char) -> Result<(usize,usize),CipherError> {
        let num = match self.alphabet.chars().position(|x| x == symbol) {
            Some(n) => n,
            None => return Err(CipherError::invalid_input_char(symbol)),
        };
        Ok((num / self.size(), num % self.size()))
    }
    
    // The inputs to this come only from internal functions that will never give invalid positions
    fn position_to_char(&self,position: (usize,usize)) -> char {
        let num = position.0*self.size() + position.1;
        self.alphabet.chars().nth(num).unwrap()
    }

    // Shift characters according to playfairs method
    fn playfair_shift(&self, lpos: (usize,usize), rpos: (usize,usize), shift: usize, size: usize, output: &mut String) {
        // The playfair_pairs() function ensures l and r never match so that case is not handled
        if lpos.0 == rpos.0 {
            let x = lpos.0;
            output.push(self.position_to_char((x, (lpos.1+shift)%size )));
            output.push(self.position_to_char((x, (rpos.1+shift)%size )));
        } else if lpos.1 == rpos.1 {
            let y = lpos.1;
            output.push(self.position_to_char(( (lpos.0+shift)%size, y )));
            output.push(self.position_to_char(( (rpos.0+shift)%size, y )));
        } else {
            output.push(self.position_to_char((lpos.0, rpos.1) ));
            output.push(self.position_to_char((rpos.0, lpos.1) ));
        }
    }

}

impl Default for Playfair {
    fn default() -> Self {
        Self{ alphabet: String::from("ABCDEFGHIJKLMNOPRSTUVWXYZ"), spacer: 'X' }
    }
}

impl Cipher for Playfair {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        self.validate_settings()?;
        let pairs = self.pairs(text)?;
        let mut out = String::with_capacity(text.chars().count());
        let size = self.size();
        let s = size+1;
        for (l,r) in pairs {
            let lpos = self.char_to_position(l)?;
            let rpos = self.char_to_position(r)?;
            
            // The playfair_pairs() function ensures l and r never match
            self.playfair_shift(lpos, rpos, size, s, &mut out);
        }
        Ok(out)
    }
    
    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        self.validate_settings()?;
        let pairs = self.pairs(text)?;
        let mut out = String::with_capacity(text.chars().count());
        let size = self.size();
        let s = size-1;
        for (l,r) in pairs {
            let lpos = self.char_to_position(l)?;
            let rpos = self.char_to_position(r)?;
            
            // The playfair_pairs() function ensures l and r never match
            self.playfair_shift(lpos, rpos, size, s, &mut out);
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.alphabet = shuffled_str(&self.alphabet, rng)
    }

    fn get_input_alphabet(&mut self) -> &String {
        &mut self.alphabet
    }

    fn get_output_alphabet(&mut self) -> &String {
        &mut self.alphabet
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn get_mut_output_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn validate_settings(&self) -> Result<(),crate::errors::CipherErrors> {
        todo!()
    }
}


impl fmt::Display for Playfair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut square = String::new();
        for (n, c) in self.alphabet.chars().enumerate() {
            if n % self.grid_size() == 0 {
                square.push_str("\n")
            }
            square.push_str(&format!("{} ",c))
        };
        write!(f, "{}", square)
    }
}