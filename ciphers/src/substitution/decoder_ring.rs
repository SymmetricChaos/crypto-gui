use crate::{errors::CipherError, traits::Cipher};
use utils::vecstring::VecString;

pub struct DecoderRing {
    pub index: usize,
    pub alphabet: VecString,
}

impl DecoderRing {
    pub fn assign_alphabet(&mut self, alphabet: &str) {
        self.alphabet = VecString::unique_from(alphabet)
    }

    pub fn length(&self) -> usize {
        self.alphabet.len()
    }

    fn valid_code_group(&self, s: &str) -> Result<usize, CipherError> {
        match s.parse::<usize>() {
            Ok(n) => {
                if n < self.length() {
                    Ok(n)
                } else {
                    Err(CipherError::invalid_input_group(s))
                }
            }
            Err(_) => return Err(CipherError::invalid_input_group(s)),
        }
    }
}

impl Default for DecoderRing {
    fn default() -> Self {
        Self {
            index: 0,
            alphabet: VecString::from("_ABCDEFGHIJKLMNOPWRSTUVWXYZ"),
        }
    }
}

impl Cipher for DecoderRing {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let symbols = text.chars();
        let mut out = Vec::new();
        for s in symbols {
            let pos = self.alphabet.get_pos(s);
            let n = match pos {
                Some(v) => (v + self.index) % self.length(),
                None => return Err(CipherError::invalid_input_char(s)),
            };
            out.push(format!("{}", n))
        }
        Ok(out.join(" "))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
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
            out.push(*self.alphabet.get_char(n).unwrap());
        }
        Ok(out)
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
        cipher.assign_alphabet("_ASLWIMVHFKXDPOEJBTNQZGUYRC");
        cipher.index = 3;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = DecoderRing::default();
        cipher.assign_alphabet("_ASLWIMVHFKXDPOEJBTNQZGUYRC");
        cipher.index = 3;
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
