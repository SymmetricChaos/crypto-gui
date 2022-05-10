use super::Cipher;
use crate::{
    errors::CipherError,
    text_aux::{
        keyed_alphabet, shuffled_str, validate_alphabet, Alphabet,
    },
};
use itertools::Itertools;
use num::integer::Roots;
use rand::prelude::StdRng;

fn is_power_of_three(a: usize) -> bool {
    let mut p = 1;
    while p < a {
        if a == p {
            return true
        } else {
            p *= 3
        }
    }
    false
}

pub struct PolybiusCube {
    pub alphabet_string: String,
    alphabet: Alphabet,
    labels: String,
    side_len: usize,
    pub key_word: String,
}

impl Default for PolybiusCube {
    fn default() -> Self {
        let alphabet = Alphabet::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ+");
        Self { 
            alphabet_string: "ABCDEFGHIJKLMNOPQRSTUVWXYZ+".to_string(), 
            alphabet, 
            labels: "123".to_string(), 
            side_len: 3, 
            key_word: String::new() }
    }
}

impl PolybiusCube {
    pub fn alphabet(&self) -> &str {
        &self.alphabet_string
    }

    pub fn assign_key(&mut self, key_word: &str) {
        self.key_word = key_word.to_string();
        self.alphabet = Alphabet::from(keyed_alphabet(&self.key_word, &self.alphabet_string));
    }

    pub fn set_key(&mut self) {
        self.alphabet = Alphabet::from(keyed_alphabet(&self.key_word, &self.alphabet_string));
    }

    pub fn set_alphabet(&mut self) -> Result<(),CipherError> {

        let new_alpha_len = self.alphabet_string.chars().count();

        if new_alpha_len > 125 {
            return Err(CipherError::alphabet("alphabet length currently limited to 125 characters"))
        }

        if !is_power_of_three(self.alphabet_string.chars().count()) {
            return Err(CipherError::alphabet("alphabet length must be a power of three"))
        }

        self.alphabet = Alphabet::from(&self.alphabet_string);
        self.side_len = self.alphabet_string.chars().count().cbrt();

        Ok(())
    }

    pub fn set_labels(&mut self, labels: String) {
        self.labels = labels
    }

    pub fn get_labels(&self) -> &String {
        &self.labels
    }

    fn triplets(&self, text: &str) -> Result<Vec<(char,char,char)>, CipherError> {
        if text.chars().count() % 3 != 0 {
            dbg!(text);
            dbg!(text.chars().count());
            return Err(CipherError::input(
                "ciphertext length must be a multiple of three.",
            ));
        }
        let out = text
            .chars()
            .chunks(3)
            .into_iter()
            .map(|x| x.collect_tuple().unwrap())
            .collect();
        Ok(out)
    }

    pub fn alphabet_len(&self) -> usize {
        self.alphabet.len()
    }

    fn char_to_position(&self, symbol: char) -> Result<(usize,usize,usize), CipherError> {
        let num = match self.alphabet_string.chars().position(|x| x == symbol) {
            Some(n) => n,
            None => return Err(CipherError::invalid_input_char(symbol)),
        };
        let l = self.side_len;
        let x = num % l;
        let y = (num % (l*l)) / l;
        let z = num / (l*l);
        Ok((x,y,z))
    }

    fn position_to_char(&self, position: (char, char, char)) -> char {
        let z = self.labels.chars().position(|c| c == position.0).unwrap();
        let y = self.labels.chars().position(|c| c == position.1).unwrap();
        let x = self.labels.chars().position(|c| c == position.2).unwrap();

        let l = self.side_len;
        let num = z * (l*l) + y * l + x;
        self.alphabet.get_char_at(num).unwrap()
    }

    fn _validate_settings(&self) -> Result<(), CipherError> {
        validate_alphabet(&self.alphabet_string)?;
        Ok(())
    }
}


impl Cipher for PolybiusCube {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut out = String::with_capacity(text.chars().count() * 2);

        for c in text.chars() {
            let pos = self.char_to_position(c)?;
            out.push(self.labels.chars().nth(pos.0).unwrap());
            out.push(self.labels.chars().nth(pos.1).unwrap());
            out.push(self.labels.chars().nth(pos.2).unwrap());
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let pairs = self.triplets(text)?;
        let mut out = String::with_capacity(text.chars().count() / 3);

        for p in pairs {
            out.push(self.position_to_char(p));
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut StdRng) {
        self.key_word = shuffled_str(&self.alphabet_string, rng)
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

// impl fmt::Display for PolybiusCube {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let mut square = String::from("  ");
//         for xlab in self.labels.chars().take(self.grid_side_len) {
//             square.push_str(&format!("{xlab} "))
//         }
//         for (n, c) in self.inner_alphabet.chars().enumerate() {
//             if n % self.grid_side_len == 0 {
//                 let ylab = self.labels.chars().nth(n / self.grid_side_len).unwrap();
//                 square.push_str(&format!("\n{ylab} "));
//             }
//             square.push_str(&format!("{c} "))
//         }
//         write!(f, "{square}")
//     }
// }

#[cfg(test)]
mod polybius_cube_tests {
    use super::*;

    const PLAINTEXT: &'static str = "ABCD";
    const CIPHERTEXT: &'static str ="";

    #[test]
    fn encrypt_test() {
        let mut cipher = PolybiusCube::default();
        //cipher.assign_key("INVENTORY");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = PolybiusCube::default();
        cipher.assign_key("INVENTORY");
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
