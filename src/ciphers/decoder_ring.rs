use rand::{Rng, prelude::ThreadRng};
use super::Cipher;
use crate::errors::CipherError;

pub struct DecoderRing {
    pub index: usize,
    pub alphabet: String,
}

impl DecoderRing {

    pub fn new(index: usize, alphabet: &str) -> Self {
        DecoderRing{ index, alphabet: alphabet.to_string() }
    }

    pub fn length(&self) -> usize {
        self.alphabet.chars().count()
    }

    pub fn annie(&mut self) {
        self.alphabet = String::from("_ASLWIMVHFKXDPOEJBTNQZGUYRC");
    }

    pub fn midnight(&mut self) {
        self.alphabet = String::from("_AEXDTZKNYCJWSGUMBOQHRIVFPL");
    }

    fn valid_code_group(&self, s: &str) -> Result<usize, CipherError> {
        match s.parse::<usize>() {
            Ok(n) => if n < self.length() { Ok(n) } else { Err(CipherError::input("invalid code group")) },
            Err(_) => return Err(CipherError::input("invalid code group")),
        }
    }
}

impl Default for DecoderRing {
    fn default() -> Self {
        Self { index: 0, alphabet: String::from("_ABCDEFGHIJKLMNOPWRSTUVWXYZ") }
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

    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.index = rng.gen_range(0..self.alphabet.len());
    }

    fn get_input_alphabet(&mut self) -> &String {
        &mut self.alphabet
    }

    fn get_output_alphabet(&mut self) -> &String {
        todo!("output alphabet should be digits and spaces")
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn get_mut_output_alphabet(&mut self) -> &mut String {
        todo!("output alphabet should be digits and spaces")
    }

    fn validate_settings(&self) -> Result<(),crate::errors::CipherErrors> {
        todo!()
    }
}




#[cfg(test)]
mod decoder_ring_tests {

    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "21 11 18 23 26 8 2 13 20 1 17 7 22 12 17 14 19 26 9 16 5 17 10 18 1 21 11 18 6 4 24 0 15 17 25";

    #[test]
    fn encrypt_test() {
        let cipher = DecoderRing::new(3,"_ASLWIMVHFKXDPOEJBTNQZGUYRC");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let cipher = DecoderRing::new(3,"_ASLWIMVHFKXDPOEJBTNQZGUYRC");
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}