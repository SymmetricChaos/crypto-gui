use crate::{errors::CipherError, traits::Cipher};
use itertools::Itertools;
use utils::{preset_alphabet::PresetAlphabet, vecstring::VecString};

pub struct FourSquare {
    pub alphabet: VecString,
    square1: VecString,
    square2: VecString,
    grid_side_len: usize,
}

impl Default for FourSquare {
    fn default() -> Self {
        Self {
            alphabet: VecString::from(PresetAlphabet::BasicLatinNoQ),
            square1: VecString::from(PresetAlphabet::BasicLatinNoQ),
            square2: VecString::from(PresetAlphabet::BasicLatinNoQ),
            grid_side_len: 5,
        }
    }
}

impl FourSquare {
    pub fn assign_key1(&mut self, key_word: &str) {
        self.square1 = VecString::keyed_alphabet(key_word, &self.alphabet.to_string());
    }

    pub fn assign_key2(&mut self, key_word: &str) {
        self.square2 = VecString::keyed_alphabet(key_word, &self.alphabet.to_string());
    }

    pub fn pick_alphabet(&mut self, mode: PresetAlphabet) {
        match mode {
            PresetAlphabet::BasicLatinNoJ
            | PresetAlphabet::BasicLatinNoQ
            | PresetAlphabet::BasicLatinWithDigits
            | PresetAlphabet::Base64 => {
                self.alphabet = VecString::from(mode);
                self.grid_side_len = (mode.len() as f64).sqrt().ceil() as usize;
            }
            _ => (),
        }
    }

    pub fn grid_side_len(&self) -> usize {
        self.grid_side_len
    }

    fn pairs(&self, text: &str) -> Vec<(char, char)> {
        text.chars()
            .collect_vec()
            .chunks(2)
            .map(|x| (x[0], x[1]))
            .collect_vec()
    }

    fn char_to_position(
        &self,
        symbol: char,
        alphabet: &VecString,
    ) -> Result<(usize, usize), CipherError> {
        let num = match alphabet.get_pos_of(symbol) {
            Some(n) => n,
            None => return Err(CipherError::invalid_input_char(symbol)),
        };
        Ok((num / self.grid_side_len, num % self.grid_side_len))
    }

    fn position_to_char(&self, position: (usize, usize), alphabet: &VecString) -> char {
        let num = position.0 * self.grid_side_len + position.1;
        alphabet.get_char_at(num).unwrap()
    }

    pub fn show_square1(&self) -> String {
        let mut out = String::new();
        for (n, c) in self.square1.chars().enumerate() {
            if n % self.grid_side_len == 0 {
                out.push_str("\n")
            }
            out.push_str(&format!("{} ", c))
        }
        out
    }

    pub fn show_square2(&self) -> String {
        let mut out = String::new();
        for (n, c) in self.square2.chars().enumerate() {
            if n % self.grid_side_len == 0 {
                out.push_str("\n")
            }
            out.push_str(&format!("{} ", c))
        }
        out
    }
}

impl Cipher for FourSquare {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let pairs = self.pairs(text);
        let mut out = String::with_capacity(text.len());
        for (l, r) in pairs {
            let lpos = self.char_to_position(l, &self.alphabet)?;
            let rpos = self.char_to_position(r, &self.alphabet)?;
            // Unlike Playfair and Two Square the Four Square cipher has no special cases to handle
            out.push(self.position_to_char((lpos.0, rpos.1), &self.square1));
            out.push(self.position_to_char((rpos.0, lpos.1), &self.square2));
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let pairs = self.pairs(text);
        let mut out = String::with_capacity(text.len());
        for (l, r) in pairs {
            let lpos = self.char_to_position(l, &self.square1)?;
            let rpos = self.char_to_position(r, &self.square2)?;
            // Unlike Playfair and Two Square the Four Square cipher has no special cases to handle
            out.push(self.position_to_char((lpos.0, rpos.1), &self.alphabet));
            out.push(self.position_to_char((rpos.0, lpos.1), &self.alphabet));
        }
        Ok(out)
    }
}

#[cfg(test)]
mod four_square_tests {
    use super::*;

    // Note the X used as padding
    const PLAINTEXT: &'static str = "HELPMEOBIWANKENOBI";
    const CIPHERTEXT: &'static str = "FYGMKYHOBXMFKKKIMD";

    #[test]
    fn encrypt_test() {
        let mut cipher = FourSquare::default();
        cipher.assign_key1("EXAMPLE");
        cipher.assign_key2("KEYWORD");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = FourSquare::default();
        cipher.assign_key1("EXAMPLE");
        cipher.assign_key2("KEYWORD");
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
