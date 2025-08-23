use super::PolyMode;
use crate::traits::Cipher;
use std::collections::VecDeque;
use utils::{errors::GeneralError, preset_alphabet::Alphabet, vecstring::VecString};

pub struct Vigenere {
    pub keywords: Vec<String>,
    pub alphabet: VecString,
    pub prog_shift: usize,
    pub mode: PolyMode,
    pub multikey: bool,
}

impl Default for Vigenere {
    fn default() -> Self {
        Self {
            keywords: vec![String::new(), String::new(), String::new()],
            alphabet: VecString::from(Alphabet::BasicLatin),
            mode: PolyMode::CylicKey,
            prog_shift: 0,
            multikey: false,
        }
    }
}

impl Vigenere {
    // Some weirdness needed to make types match
    pub fn key(&self) -> impl Iterator<Item = usize> + '_ {
        if self.multikey {
            let mut effective_key = vec![0usize; self.key_len()];
            for key in self.keywords.iter().filter(|s| !s.is_empty()) {
                for (pos, sym) in key.chars().cycle().take(self.key_len()).enumerate() {
                    let p = self.alphabet.get_pos(sym).unwrap();
                    effective_key[pos] += p
                }
            }
            effective_key = effective_key
                .into_iter()
                .map(|v| v % self.alphabet_len())
                .collect();
            effective_key.into_iter()
        } else {
            let key: Vec<usize> = self.keywords[0]
                .chars()
                .map(|x| self.alphabet.get_pos(x).unwrap())
                .collect();
            key.into_iter()
        }
    }

    pub fn cyclic_key(&self) -> impl Iterator<Item = usize> + '_ {
        let v = self.key().collect::<Vec<usize>>();
        v.into_iter().cycle()
    }

    //Should multiply together ignoring common factors. [9,6] should give 18
    pub fn key_len(&self) -> usize {
        if self.multikey {
            self.keywords
                .iter()
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().count())
                .fold(1, num::integer::lcm)
        } else {
            self.keywords[0].chars().count()
        }
    }

    // Unwrap justified by bounds on key
    pub fn keyword(&self) -> String {
        if self.multikey {
            self.key()
                .map(|v| self.alphabet.get_char(v).unwrap())
                .collect()
        } else {
            self.keywords[0].clone()
        }
    }

    pub fn alphabet_len(&self) -> usize {
        self.alphabet.len()
    }

    fn validate_key(&self) -> Result<(), GeneralError> {
        for key in self.keywords.iter() {
            for c in key.chars() {
                if !self.alphabet.contains(c) {
                    return Err(GeneralError::invalid_alphabet_char(c));
                }
            }
        }
        Ok(())
    }

    fn validate_input(&self, text: &str) -> Result<(), GeneralError> {
        if text.len() == 0 {
            return Err(GeneralError::input("No input text provided"));
        }
        for c in text.chars() {
            if !self.alphabet.contains(c) {
                return Err(GeneralError::invalid_input_char(c));
            }
        }
        Ok(())
    }

    pub fn assign_alphabet(&mut self, alphabet: &str) {
        self.alphabet = VecString::unique_from(&alphabet);
    }

    // Unwraps for the character methods are justified by validating the input
    fn encrypt_char(&self, c: char, k: usize) -> Result<char, GeneralError> {
        let p = self
            .alphabet
            .get_pos(c)
            .ok_or(GeneralError::invalid_input_char(c))?;
        Ok(*self.alphabet.get_char_offset(p, k as i32).unwrap())
    }

    fn decrypt_char(&self, c: char, k: usize) -> Result<char, GeneralError> {
        let p = self
            .alphabet
            .get_pos(c)
            .ok_or(GeneralError::invalid_input_char(c))?;
        Ok(*self.alphabet.get_char_offset(p, -(k as i32)).unwrap())
    }

    fn encrypt_cyclic(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = String::with_capacity(text.len());
        for (c, n) in text.chars().zip(self.cyclic_key()) {
            out.push(self.encrypt_char(c, n)?)
        }
        Ok(out)
    }

    fn decrypt_cyclic(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = String::with_capacity(text.len());
        for (c, n) in text.chars().zip(self.cyclic_key()) {
            out.push(self.decrypt_char(c, n)?)
        }
        Ok(out)
    }

    fn encrypt_auto(&self, text: &str) -> Result<String, GeneralError> {
        let mut akey: VecDeque<usize> = self.key().collect();
        let mut out = String::with_capacity(text.len());

        for c in text.chars() {
            akey.push_back(self.alphabet.get_pos(c).unwrap());
            let n = akey.pop_front().unwrap();
            out.push(self.encrypt_char(c, n)?)
        }

        Ok(out)
    }

    fn decrypt_auto(&self, text: &str) -> Result<String, GeneralError> {
        let mut akey: VecDeque<usize> = self.key().collect();
        let mut out = String::with_capacity(text.len());

        for c in text.chars() {
            let n = akey.pop_front().unwrap();
            let ptxt_char = self.decrypt_char(c, n)?;
            out.push(ptxt_char);
            let new_key_val = self.alphabet.get_pos(ptxt_char).unwrap();
            akey.push_back(new_key_val);
        }
        Ok(out)
    }

    fn encrypt_prog(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = String::with_capacity(text.len());

        let mut cur_shift = 0 as usize;
        let mut ctr = 0;
        let key_len = self.key_len();

        for (c, n) in text.chars().zip(self.cyclic_key()) {
            out.push(self.encrypt_char(c, (n + cur_shift) % self.alphabet_len())?);
            ctr = (ctr + 1) % key_len;
            if ctr == 0 {
                cur_shift = (cur_shift + self.prog_shift) % self.alphabet_len();
            }
        }
        Ok(out)
    }

    fn decrypt_prog(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = String::with_capacity(text.len());

        let mut cur_shift = 0;
        let mut ctr = 0;
        let key_len = self.key_len();

        for (c, n) in text.chars().zip(self.cyclic_key()) {
            out.push(self.decrypt_char(c, (n + cur_shift) % self.alphabet_len())?);
            ctr = (ctr + 1) % key_len;
            if ctr == 0 {
                cur_shift = (cur_shift + self.prog_shift) % self.alphabet_len();
            }
        }
        Ok(out)
    }
}

impl Cipher for Vigenere {
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        self.validate_key()?;
        self.validate_input(text)?;
        match self.mode {
            PolyMode::CylicKey => self.encrypt_cyclic(text),
            PolyMode::Autokey => self.encrypt_auto(text),
            PolyMode::ProgKey => self.encrypt_prog(text),
        }
    }

    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
        self.validate_key()?;
        self.validate_input(text)?;
        match self.mode {
            PolyMode::CylicKey => self.decrypt_cyclic(text),
            PolyMode::Autokey => self.decrypt_auto(text),
            PolyMode::ProgKey => self.decrypt_prog(text),
        }
    }

    // fn randomize(&mut self) {
    //     let rng = &mut get_global_rng();
    //     let alpha = String::from(&self.alphabet_string);
    //     self.keywords[0] = random_sample_replace(&alpha, 3, rng);
    //     self.keywords[1] = random_sample_replace(&alpha, 4, rng);
    //     self.keywords[2] = random_sample_replace(&alpha, 5, rng);
    //     self.keywords[3] = String::new();
    //     self.keywords[4] = String::new();
    // }
}

#[cfg(test)]
mod vigenere_tests {
    use super::*;

    const PLAINTEXT: &'static str =         "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGANDTHENSOMEMORETOMAKEALONGERPLAINTEXTFORTHISCIPHERTOUSE";
    const CIPHERTEXT_CYCLIC: &'static str = "XUGHSXVSPESJPWMMCCACWBXVPIAMZNDLFFEPGLHUIAUFKTFWFRXBORITTTCAKRTGJPBVHRBGHFPIAQGPMCJVPIHCGR";
    const CIPHERTEXT_AUTO: &'static str =   "XUGHSXVSPEHDRVIFLENGGKIJFQQYXPRMYSXTUHEHDLVCSEZRKLXBEOWIMZFRZSDPVEIYHRDXWDCTPVLGFIMSIMVCKG";
    const CIPHERTEXT_PROG: &'static str =   "XUGHSXVSPEVMSZPPFFDFCHDBVOGSFTMUOONYPUQDUMGRWFRIRDMQDGXIIIRPCJLYBHTNZJWBCAKDVLBKKAHTNGFAEP";

    #[test]
    fn encrypt_test_cyclic() {
        let mut cipher = Vigenere::default();
        cipher.keywords[1] = String::from("GOOD");
        cipher.keywords[0] = String::from("ENCRYPTION");
        cipher.mode = PolyMode::CylicKey;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_CYCLIC);
    }

    #[test]
    fn decrypt_test_cyclic() {
        let mut cipher = Vigenere::default();
        cipher.keywords[1] = String::from("GOOD");
        cipher.keywords[0] = String::from("ENCRYPTION");
        cipher.mode = PolyMode::CylicKey;
        assert_eq!(cipher.decrypt(CIPHERTEXT_CYCLIC).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_auto() {
        let mut cipher = Vigenere::default();
        cipher.keywords[1] = String::from("GOOD");
        cipher.keywords[0] = String::from("ENCRYPTION");
        cipher.mode = PolyMode::Autokey;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_AUTO);
    }

    #[test]
    fn decrypt_test_auto() {
        let mut cipher = Vigenere::default();
        cipher.keywords[1] = String::from("GOOD");
        cipher.keywords[0] = String::from("ENCRYPTION");
        cipher.mode = PolyMode::Autokey;
        assert_eq!(cipher.decrypt(CIPHERTEXT_AUTO).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_prog() {
        let mut cipher = Vigenere::default();
        cipher.keywords[1] = String::from("GOOD");
        cipher.keywords[0] = String::from("ENCRYPTION");
        cipher.mode = PolyMode::ProgKey;
        cipher.prog_shift = 3;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_PROG);
    }

    #[test]
    fn decrypt_test_prog() {
        let mut cipher = Vigenere::default();
        cipher.keywords[1] = String::from("GOOD");
        cipher.keywords[0] = String::from("ENCRYPTION");
        cipher.mode = PolyMode::ProgKey;
        cipher.prog_shift = 3;
        assert_eq!(cipher.decrypt(CIPHERTEXT_PROG).unwrap(), PLAINTEXT);
    }
}
