use rand::Rng;

use crate::global_rng::get_global_rng;
use crate::text_aux::VecString;
use crate::{ciphers::Cipher, errors::Error};

pub struct DecoderRing {
    pub index: usize,
    alphabet: VecString,
    pub alphabet_string: String,
}

impl DecoderRing {
    pub fn set_alphabet(&mut self) {
        self.alphabet = VecString::unique_from(&self.alphabet_string)
    }

    pub fn control_alphabet(&mut self) -> &mut String {
        self.alphabet = VecString::unique_from(&self.alphabet_string);
        &mut self.alphabet_string
    }

    pub fn length(&self) -> usize {
        self.alphabet.len()
    }

    pub fn annie(&mut self) {
        self.alphabet_string = String::from("_ASLWIMVHFKXDPOEJBTNQZGUYRC");
        self.alphabet = VecString::from(&self.alphabet_string);
    }

    pub fn midnight(&mut self) {
        self.alphabet_string = String::from("_AEXDTZKNYCJWSGUMBOQHRIVFPL");
        self.alphabet = VecString::from(&self.alphabet_string);
    }

    fn valid_code_group(&self, s: &str) -> Result<usize, Error> {
        match s.parse::<usize>() {
            Ok(n) => {
                if n < self.length() {
                    Ok(n)
                } else {
                    Err(Error::input("invalid code group"))
                }
            }
            Err(_) => return Err(Error::input("invalid code group")),
        }
    }
}

impl Default for DecoderRing {
    fn default() -> Self {
        Self {
            index: 0,
            alphabet_string: String::from("_ABCDEFGHIJKLMNOPWRSTUVWXYZ"),
            alphabet: VecString::from("_ABCDEFGHIJKLMNOPWRSTUVWXYZ"),
        }
    }
}

impl Cipher for DecoderRing {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        let symbols = text.chars();
        let mut out = Vec::new();
        for s in symbols {
            let pos = self.alphabet.get_pos_of(s);
            let n = match pos {
                Some(v) => (v + self.index) % self.length(),
                None => return Err(Error::invalid_input_char(s)),
            };
            out.push(format!("{}", n))
        }
        Ok(out.join(" "))
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        let code_groups = text.split(' ');
        let nums = {
            let mut v = Vec::with_capacity(code_groups.clone().count());
            for s in code_groups {
                let n = self.valid_code_group(s)?;
                v.push((n + self.length() - self.index) % self.length());
            }
            v
        };
        let mut out = String::with_capacity(nums.len());
        for n in nums {
            // Unwrap is justified by the valid_code_groups method which catches both possibles sorts of errors
            out.push(self.alphabet.get_char_at(n).unwrap());
        }
        Ok(out)
    }

    fn randomize(&mut self) {
        self.index = get_global_rng().gen_range(0..self.alphabet.len());
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod decoder_ring_tests {

    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "21 11 18 23 26 8 2 13 20 1 17 7 22 12 17 14 19 26 9 16 5 17 10 18 1 21 11 18 6 4 24 0 15 17 25";

    // _ A S L W I M V H F  K  X  D  P  O  E  J  B  T  N  Q  Z  G  U  Y  R  C
    // 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26

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
