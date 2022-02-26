use rand::prelude::ThreadRng;
use crate::text_functions::{shuffled_str, dedup_alphabet};
use super::Cipher;
use crate::errors::CipherError;
use crate::text_types::PresetAlphabet;

pub struct GeneralSubstitution {
    alphabet1: String,
    alphabet2: String,
}

impl GeneralSubstitution {

    pub fn set_alphabets(&mut self, mode: PresetAlphabet) {
        self.alphabet1 = String::from(mode);
        self.alphabet2 = String::from(mode);
    }

    pub fn control_alphabet1(&mut self) -> &mut String {
        self.alphabet1 = dedup_alphabet(&self.alphabet1);
        &mut self.alphabet1
    }

    pub fn control_alphabet2(&mut self) -> &mut String {
        self.alphabet2 = dedup_alphabet(&self.alphabet2);
        &mut self.alphabet2
    }

    pub fn encrypt_char(&self, c: char) -> char {
        let pos = self.alphabet1.chars().position(|x| x == c).unwrap();
        self.alphabet2.chars().nth(pos).unwrap()
    }

    pub fn decrypt_char(&self, c: char) -> char {
        let pos = self.alphabet2.chars().position(|x| x == c).unwrap();
        self.alphabet1.chars().nth(pos).unwrap()
    }
}

impl Default for GeneralSubstitution {
    fn default() -> Self {
        let alphabet1 = String::from(PresetAlphabet::BasicLatin);
        let alphabet2 = String::from(PresetAlphabet::BasicLatin);
        Self { alphabet1, alphabet2 }
    }
}

impl Cipher for GeneralSubstitution {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        self.validate_settings()?;
        let out = text.chars().map(|c| self.encrypt_char(c)).collect();
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        self.validate_settings()?;
        let out = text.chars().map(|c| self.decrypt_char(c)).collect();
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.alphabet2 = shuffled_str(&self.alphabet1, rng);
    }

    fn get_input_alphabet(&self) -> &String {
        &self.alphabet1
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet1
    }

    fn validate_settings(&self) -> Result<(), CipherError> {
        if self.alphabet1.chars().count() != self.alphabet2.chars().count() {
            return Err(CipherError::key("the input and output alphabets must have the same length"))
        }
        Ok(())
    }
    
    fn reset(&mut self) {
        *self = Self::default();
    }
}



#[cfg(test)]
mod gen_sub_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT1: &'static str = "ALGVBMUCDTRXWPRNEBJHYRZGTALGFOIQSRK";
    const CIPHERTEXT2: &'static str = "🐏🚚📢🔝😩💡🆚🚅🍥☪🕳🚆🌃🍒🕳🚢🎴😩⏳👈📡🕳🕘📢☪🐏🚚📢😽⏯🚪😪💲🕳💮";

    #[test]
    fn encrypt_test1() {
        let mut cipher = GeneralSubstitution::default();
        *cipher.control_alphabet2() = String::from("ODUSGPKLMECFJWRHVTYABZXNQI");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT1);
    }

    #[test]
    fn decrypt_test1() {
        let mut cipher = GeneralSubstitution::default();
        *cipher.control_alphabet2() = String::from("ODUSGPKLMECFJWRHVTYABZXNQI");
        assert_eq!(cipher.decrypt(CIPHERTEXT1).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test2() {
        let mut cipher = GeneralSubstitution::default();
        *cipher.control_alphabet2() = String::from("⏯🍥🆚💲📢🍒💮🚚💡🎴🚅😽⏳🌃🕳👈🔝☪📡🐏😩🕘🚆🚢😪🚪");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT2);
    }

    #[test]
    fn decrypt_test2() {
        let mut cipher = GeneralSubstitution::default();
        *cipher.control_alphabet2() = String::from("⏯🍥🆚💲📢🍒💮🚚💡🎴🚅😽⏳🌃🕳👈🔝☪📡🐏😩🕘🚆🚢😪🚪");
        assert_eq!(cipher.decrypt(CIPHERTEXT2).unwrap(), PLAINTEXT);
    }
}