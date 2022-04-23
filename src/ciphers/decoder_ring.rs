use rand::{Rng, prelude::StdRng};
use super::Cipher;
use crate::errors::CipherError;
use crate::text_aux::Alphabet;

pub struct DecoderRing {
    pub index: usize,
    alphabet: Alphabet,
    pub alphabet_string: String,
}

impl DecoderRing {

    pub fn control_alphabet(&mut self) -> &mut String {
        self.alphabet = Alphabet::from(&self.alphabet_string);
        &mut self.alphabet_string
    }

    pub fn length(&self) -> usize {
        self.alphabet.chars().count()
    }

    pub fn annie(&mut self) {
        self.alphabet_string = String::from("_ASLWIMVHFKXDPOEJBTNQZGUYRC");
        self.alphabet = Alphabet::from(&self.alphabet_string);
    }

    pub fn midnight(&mut self) {
        self.alphabet_string = String::from("_AEXDTZKNYCJWSGUMBOQHRIVFPL");
        self.alphabet = Alphabet::from(&self.alphabet_string);
    }

    fn valid_code_group(&self, s: &str) -> Result<usize, CipherError> {
        match s.parse::<usize>() {
            Ok(n) => if n < self.length() { 
                    Ok(n) 
                } else { 
                    Err(CipherError::input("invalid code group")) 
                },
            Err(_) => return Err(CipherError::input("invalid code group")),
        }
    }
}

impl Default for DecoderRing {
    fn default() -> Self {
        Self { index: 0, alphabet_string: String::from("_ABCDEFGHIJKLMNOPWRSTUVWXYZ"), alphabet: Alphabet::from("_ABCDEFGHIJKLMNOPWRSTUVWXYZ") }
    }
}

impl Cipher for DecoderRing {

    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        let symbols = text.chars();
        let mut out = Vec::new();
        for s in symbols {
            let pos = self.alphabet.chars().position(|x| x == s);
            let n = match pos {
                Some(v) => (v + self.index) % self.length(),
                None => return Err(CipherError::invalid_input_char(s))
            };
            out.push( format!("{}",n) )
        }
        Ok(out.join(" "))
    }

    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        let code_groups = text.split(' ');
        let nums =  {
            let mut v = Vec::with_capacity(code_groups.clone().count());
            for s in code_groups {
                let n = self.valid_code_group(s)?;
                v.push( (n + self.length() - self.index) % self.length());
            }
            v
        };
        let mut out = String::with_capacity(nums.len());
        for n in nums {
            // Unwrap is justified by the valid_code_groups method which catches both possibles sorts of errors
            out.push(self.alphabet.chars().nth(n).unwrap());
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut StdRng) {
        self.index = rng.gen_range(0..self.alphabet.len());
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}




#[cfg(test)]
mod decoder_ring_tests {

    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "22 10 7 19 23 11 5 13 4 20 17 25 16 8 17 26 12 23 15 18 21 17 24 7 20 22 10 7 14 3 1 0 6 17 9";

    #[test]
    fn encrypt_test() {
        let mut cipher = DecoderRing::default();
        cipher.annie();
        cipher.index = 3;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = DecoderRing::default();
        cipher.annie();
        cipher.index = 3;
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}