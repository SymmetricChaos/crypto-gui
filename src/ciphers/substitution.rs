use rand::prelude::StdRng;
use crate::{
    text_aux::{shuffled_str, PresetAlphabet, Alphabet}, 
    errors::CipherError
};
use super::Cipher;

#[derive(Debug)]
pub struct GeneralSubstitution {
    alphabet_string1: String,
    alphabet1: Alphabet,
    alphabet_string2: String,
    alphabet2: Alphabet,
}

impl GeneralSubstitution {

    pub fn _set_alphabet1(&mut self, symbols: &str) {
        self.alphabet1 = Alphabet::from(symbols);
        self.alphabet_string1 = self.alphabet1.to_string();
    }

    pub fn _set_alphabet2(&mut self, symbols: &str) {
        self.alphabet2 = Alphabet::from(symbols);
        self.alphabet_string2 = self.alphabet2.to_string();
    }

    pub fn control_alphabet1(&mut self) -> &mut String {
        self.alphabet1 = Alphabet::from(&self.alphabet_string1);
        &mut self.alphabet_string1
    }

    pub fn control_alphabet2(&mut self) -> &mut String {
        self.alphabet2 = Alphabet::from(&self.alphabet_string2);
        &mut self.alphabet_string2
    }

    pub fn encrypt_char(&self, c: char) -> char {
        let pos = self.alphabet1.get_pos(c).unwrap();
        self.alphabet2.get_char(pos).unwrap()
    }

    pub fn decrypt_char(&self, c: char) -> char {
        let pos = self.alphabet2.get_pos(c).unwrap();
        self.alphabet1.get_char(pos).unwrap()
    }

    fn validate_settings(&self) -> Result<(), CipherError> {
        if self.alphabet1.chars().count() != self.alphabet2.chars().count() {
            return Err(CipherError::key("the input and output alphabets must have the same length"))
        }
        Ok(())
    }

    fn validate_text_encrypt(&self, text: &str) -> Result<(), CipherError> {
        for c in text.chars() {
            if !self.alphabet1.contains(c) {
                return Err(CipherError::invalid_input_char(c))
            }
        }
        Ok(())
    }

    fn validate_text_decrypt(&self, text: &str) -> Result<(), CipherError> {
        for c in text.chars() {
            if !self.alphabet2.contains(c) {
                return Err(CipherError::invalid_input_char(c))
            }
        }
        Ok(())
    }
}

impl Default for GeneralSubstitution {
    fn default() -> Self {
        let alphabet_string1 = String::from(PresetAlphabet::BasicLatin);
        let alphabet1 = Alphabet::from(&alphabet_string1);
        let alphabet_string2 = String::from("ZYXWVUTSRQPONMLKJIHGFEDCBA");
        let alphabet2 = Alphabet::from(&alphabet_string2);
        Self { alphabet1, alphabet_string1, alphabet2, alphabet_string2 }
    }
}

impl Cipher for GeneralSubstitution {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        self.validate_settings()?;
        self.validate_text_encrypt(text)?;
        let out = text.chars().map(|c| self.encrypt_char(c)).collect();
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        self.validate_settings()?;
        self.validate_text_decrypt(text)?;
        let out = text.chars().map(|c| self.decrypt_char(c)).collect();
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut StdRng) {
        self.alphabet_string2 = shuffled_str(&self.alphabet_string1, rng);
        self.alphabet2 = Alphabet::from(&self.alphabet_string2);
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
        cipher._set_alphabet2("ODUSGPKLMECFJWRHVTYABZXNQI");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT1);
    }

    #[test]
    fn decrypt_test1() {
        let mut cipher = GeneralSubstitution::default();
        cipher._set_alphabet2("ODUSGPKLMECFJWRHVTYABZXNQI");
        assert_eq!(cipher.decrypt(CIPHERTEXT1).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test2() {
        let mut cipher = GeneralSubstitution::default();
        cipher._set_alphabet2("⏯🍥🆚💲📢🍒💮🚚💡🎴🚅😽⏳🌃🕳👈🔝☪📡🐏😩🕘🚆🚢😪🚪");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT2);
    }

    #[test]
    fn decrypt_test2() {
        let mut cipher = GeneralSubstitution::default();
        cipher._set_alphabet2("⏯🍥🆚💲📢🍒💮🚚💡🎴🚅😽⏳🌃🕳👈🔝☪📡🐏😩🕘🚆🚢😪🚪");
        assert_eq!(cipher.decrypt(CIPHERTEXT2).unwrap(), PLAINTEXT);
    }
}