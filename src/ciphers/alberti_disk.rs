use std::fmt::Display;

use crate::errors::CipherError;

use super::Cipher;

pub struct Alberti {
    fixed_alphabet: String,
    moving_alphabet: String,
    start_index: usize,
}
 
impl Alberti {
 
    fn encrypt_char(&self, symbol: char, index: usize) -> char {
        let position = self.fixed_alphabet.chars().position(|x| x == symbol).unwrap();
        self.moving_alphabet.chars().nth(position + index % self.length()).unwrap()
    }
 
    // fn decrypt_char(&self, symbol: char, index: usize) -> char {
    //     let position = self.moving_alphabet.chars().position(|x| x == symbol).unwrap();
    //     self.fixed_alphabet.chars().nth(position + index % self.length()).unwrap()
    // }
 
 
    fn length(&self) -> usize {
        self.fixed_alphabet.chars().count()
    }
 

}

impl Cipher for Alberti {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        let mut index = self.start_index.clone();
        let mut out = String::with_capacity(text.len());
        for s in text.chars() {
            if self.fixed_alphabet.contains(s) {
                out.push(self.encrypt_char(s,index));
            } else if self.moving_alphabet.contains(s) {
                index = self.moving_alphabet.chars().position(|x| x == s).unwrap();
                out.push(self.fixed_alphabet.chars().nth(index).unwrap());
            } else {
                todo!("CipherError::Input")
            }
        }
        Ok(out)
    }


    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        todo!("{}",text)
    }

    fn randomize(&mut self, rng: &mut rand::prelude::ThreadRng) {
        todo!("{:?}",rng)
    }

    fn get_input_alphabet(&mut self) -> &String {
        todo!("may change with mode")
    }

    fn get_output_alphabet(&mut self) -> &String {
        todo!("may change with mode")
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        todo!("may change with mode")
    }

    fn get_mut_output_alphabet(&mut self) -> &mut String {
        todo!("may change with mode")
    }

    fn validate_settings(&self) -> Result<(),crate::errors::CipherErrors> {
        todo!()
    }

}
 
impl Default for Alberti {
    fn default() -> Self {
        Self{ fixed_alphabet:  String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), 
              moving_alphabet: String::from("abcdefghijklmnopqrstuvwxyz"),
              start_index: 0} 
    }
}

impl Display for Alberti {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = self.fixed_alphabet.clone();
        out.push_str(&self.fixed_alphabet[self.start_index..]);
        out.push_str(&self.fixed_alphabet[0..self.start_index]);
        write!(f, "{}", out)
    }
}