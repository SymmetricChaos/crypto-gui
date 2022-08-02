use crate::{
    ciphers::Cipher,
    errors::Error,
    global_rng::get_global_rng,
    text_aux::{
        shuffled_str,
        PresetAlphabet::{self, *},
        VecString,
    },
};
use itertools::Itertools;
use num::integer::Roots;

pub struct FourSquare {
    pub alphabet: VecString,
    square1: VecString,
    square2: VecString,
    pub key_word1: String,
    pub key_word2: String,
    grid_side_len: usize,
}

impl Default for FourSquare {
    fn default() -> Self {
        Self {
            alphabet: VecString::from(PresetAlphabet::BasicLatinNoQ),
            square1: VecString::from(PresetAlphabet::BasicLatinNoQ),
            square2: VecString::from(PresetAlphabet::BasicLatinNoQ),
            key_word1: String::new(),
            key_word2: String::new(),
            grid_side_len: 5,
        }
    }
}

impl FourSquare {
    pub fn assign_key1(&mut self, key_word: &str) {
        self.key_word1 = key_word.to_string();
        self.set_key1();
    }

    pub fn assign_key2(&mut self, key_word: &str) {
        self.key_word2 = key_word.to_string();
        self.set_key2();
    }

    pub fn set_key1(&mut self) {
        self.square1 = VecString::keyed_alphabet(&self.key_word1, &self.alphabet.to_string());
    }

    pub fn set_key2(&mut self) {
        self.square2 = VecString::keyed_alphabet(&self.key_word2, &self.alphabet.to_string());
    }

    pub fn assign_alphabet(&mut self, mode: PresetAlphabet) {
        match mode {
            BasicLatinNoJ | BasicLatinNoQ | BasicLatinWithDigits | Base64 => {
                self.alphabet = VecString::from(mode);
                self.set_key1();
                self.set_key2();
                self.grid_side_len = mode.len().sqrt();
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
    ) -> Result<(usize, usize), Error> {
        let num = match alphabet.get_pos_of(symbol) {
            Some(n) => n,
            None => return Err(Error::invalid_input_char(symbol)),
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
    fn encrypt(&self, text: &str) -> Result<String, Error> {
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

    fn decrypt(&self, text: &str) -> Result<String, Error> {
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

    fn randomize(&mut self) {
        self.key_word1 = shuffled_str(&self.alphabet.to_string(), &mut get_global_rng());
        self.key_word2 = shuffled_str(&self.alphabet.to_string(), &mut get_global_rng());
        self.set_key1();
        self.set_key2();
    }

    fn reset(&mut self) {
        *self = Self::default();
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
