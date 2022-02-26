use std::fmt;
use num::integer::Roots;
use rand::prelude::ThreadRng;
use super::Cipher;
use crate::{errors::CipherError, text_functions::shuffled_str};
use crate::text_functions::{keyed_alphabet};
use crate::text_types::{PresetAlphabet::*, PresetAlphabet};

pub struct Playfair {
    alphabet: String,
    square: String,
    key_word: String,
    spacer: char,
    grid_side_len: usize,
}

impl Playfair {

    // Silently ignores invalid characters
    pub fn control_key(&mut self) -> &mut String {
        self.square = keyed_alphabet(&self.key_word, &self.alphabet);
        &mut self.key_word
    }

    pub fn set_key(&mut self, key_word: &str) {
        self.key_word = key_word.to_string();
        self.square = keyed_alphabet(key_word, &self.alphabet);
    }

    pub fn control_spacer(&mut self) -> &mut char {
        &mut self.spacer
    }

    pub fn set_alphabet(&mut self, mode: PresetAlphabet) {
        match mode {
            BasicLatinNoJ | BasicLatinNoQ | BasicLatinWithDigits | Base64 => {
                self.alphabet = mode.string();
                self.square = mode.string();
                self.grid_side_len = mode.len().sqrt();
            }
            _ => ()
        }
    }

    fn pairs(&self, text: &str) -> Vec<(char,char)> {
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
        if symbols.len() != 0 {
            out.push( (symbols.pop().unwrap(), self.spacer) )
        }
        out
    }

    fn char_to_position(&self, symbol: char) -> Result<(usize,usize),CipherError> {
        let num = match self.square.chars().position(|x| x == symbol) {
            Some(n) => n,
            None => return Err(CipherError::invalid_input_char(symbol)),
        };
        Ok((num / self.grid_side_len, num % self.grid_side_len))
    }
    
    // The inputs to this come only from internal functions that will never give invalid positions
    fn position_to_char(&self, position: (usize,usize)) -> char {
        let num = position.0*self.grid_side_len + position.1;
        self.square.chars().nth(num).unwrap()
    }

    // Shift characters according to playfairs method
    fn playfair_shift(&self, lpos: (usize,usize), rpos: (usize,usize), shift: usize, output: &mut String) {
        let size = self.grid_side_len;
        // The pairs() function ensures l and r never match so that case is not handled
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
        Self{ alphabet: String::from(PresetAlphabet::BasicLatinNoQ), 
              square: String::from(PresetAlphabet::BasicLatinNoQ), 
              spacer: 'X', 
              grid_side_len: 5, 
              key_word: String::new() }
    }
}

impl Cipher for Playfair {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        self.validate_settings()?;
        let pairs = self.pairs(text);
        let mut out = String::with_capacity(text.chars().count());
        let shift = self.grid_side_len+1;
        for (l, r) in pairs {
            let lpos = self.char_to_position(l)?;
            let rpos = self.char_to_position(r)?;
            self.playfair_shift(lpos, rpos, shift,  &mut out);
        }
        Ok(out)
    }
    
    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        self.validate_settings()?;
        let pairs = self.pairs(text);
        let mut out = String::with_capacity(text.chars().count());
        let shift = self.grid_side_len-1;
        for (l, r) in pairs {
            let lpos = self.char_to_position(l)?;
            let rpos = self.char_to_position(r)?;
            self.playfair_shift(lpos, rpos, shift, &mut out);
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.alphabet = shuffled_str(&self.alphabet, rng)
    }

    fn get_input_alphabet(&self) -> &String {
        &self.square
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        &mut self.square
    }

    fn validate_settings(&self) -> Result<(), CipherError> {
        if !&self.alphabet.contains(self.spacer) {
            return Err(CipherError::Key(format!("spacer character {} is not in the alphabet",self.spacer)))
        }
        Ok(())
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}


impl fmt::Display for Playfair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        for (n, c) in self.square.chars().enumerate() {
            if n % self.grid_side_len == 0 {
                out.push_str("\n")
            }
            out.push_str(&format!("{} ",c))
        };
        write!(f, "{}", out)
    }
}


#[cfg(test)]
mod playfair_tests {
    use super::*;

    // Note Q replaced by K and the X used as padding
    const PLAINTEXT: &'static str =  "THEKUICKBROWNFOXJUMPSOVERTHELAZYDOGX";
    const CIPHERTEXT: &'static str = "WGVOEGAOAWNXKHXEGLNKCMULTWIZVDLWCPIT";

    #[test]
    fn encrypt_test() {
        let mut cipher = Playfair::default();
        cipher.set_key("VUVUZELAS");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Playfair::default();
        cipher.set_key("VUVUZELAS");
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}