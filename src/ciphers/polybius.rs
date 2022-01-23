use num::Integer;
use rand::prelude::ThreadRng;
use super::Cipher;
use crate::text_functions::{LATIN_UPPER_NO_J, shuffled_str, DIGITS};

pub struct Square {

}

pub struct PolybiusSquare {
    alphabet: String,
    labels: Vec<char>,
    size: usize,
}

impl PolybiusSquare {
    fn char_to_val(&self, c: char) -> Option<[char;2]> {
        match self.alphabet.chars().position(|x| x == c) {
            Some(n) => {
                let x = n.div_rem(&self.size);
                Some([self.labels[x.0],self.labels[x.0]])
            },
            None => None,
        }
    }

    // fn val_to_char(&self, v: [char;2]) -> Option<char> {
    //     v[0]
    //     v[1]
    // }


}

// impl Default for PolybiusSquare {
//     fn default() -> Self {
//         Self { alphabet: String::from(LATIN_UPPER_NO_J), labels: String::from(DIGITS), size: 5 }
//     }
// }

impl Cipher for PolybiusSquare {
    fn encrypt(&self, text: &str) -> Result<String,&'static str> {
        let mut out = String::with_capacity(text.len());

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String,&'static str> {
        let mut out = String::with_capacity(text.len());

        Ok(out)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.alphabet = shuffled_str(&self.alphabet, rng);
    }

    fn input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn output_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }
}