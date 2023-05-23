use utils::{preset_alphabet::PresetAlphabet, vecstring::VecString};

use crate::{errors::CipherError, traits::Cipher};

#[derive(Debug)]
pub struct GeneralSubstitution {
    pt_alphabet: VecString,
    ct_alphabet: VecString,
}

impl GeneralSubstitution {
    pub fn assign_pt_alphabet(&mut self, alphabet: &str) {
        self.pt_alphabet = VecString::unique_from(alphabet)
    }

    pub fn assign_ct_alphabet(&mut self, alphabet: &str) {
        self.ct_alphabet = VecString::unique_from(alphabet)
    }

    pub fn encrypt_char(&self, c: char) -> char {
        let pos = self.pt_alphabet.get_pos_of(c).unwrap();
        self.ct_alphabet.get_char_at(pos).unwrap()
    }

    pub fn decrypt_char(&self, c: char) -> char {
        let pos = self.ct_alphabet.get_pos_of(c).unwrap();
        self.pt_alphabet.get_char_at(pos).unwrap()
    }

    fn validate_settings(&self) -> Result<(), CipherError> {
        if self.pt_alphabet.chars().count() != self.ct_alphabet.chars().count() {
            return Err(CipherError::key(
                "the input and output alphabets must have the same length",
            ));
        }
        Ok(())
    }

    fn validate_text_encrypt(&self, text: &str) -> Result<(), CipherError> {
        for c in text.chars() {
            if !self.pt_alphabet.contains(c) {
                return Err(CipherError::invalid_input_char(c));
            }
        }
        Ok(())
    }

    fn validate_text_decrypt(&self, text: &str) -> Result<(), CipherError> {
        for c in text.chars() {
            if !self.ct_alphabet.contains(c) {
                return Err(CipherError::invalid_input_char(c));
            }
        }
        Ok(())
    }
}

impl Default for GeneralSubstitution {
    fn default() -> Self {
        let pt_alphabet = VecString::from(PresetAlphabet::BasicLatin);
        let ct_alphabet = VecString::from("ZYXWVUTSRQPONMLKJIHGFEDCBA");
        Self {
            pt_alphabet,
            ct_alphabet,
        }
    }
}

impl Cipher for GeneralSubstitution {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        self.validate_settings()?;
        self.validate_text_encrypt(text)?;
        let out = text.chars().map(|c| self.encrypt_char(c)).collect();
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.validate_settings()?;
        self.validate_text_decrypt(text)?;
        let out = text.chars().map(|c| self.decrypt_char(c)).collect();
        Ok(out)
    }

    // fn randomize(&mut self) {
    //     // keep the plaintext alphabet unchanged and make the ciphertext alphabet a shuffled version of it
    //     self.ct_alphabet_string = shuffled_str(&self.pt_alphabet_string, &mut get_global_rng());
    //     self.ct_alphabet = VecString::unique_from(&self.ct_alphabet_string);
    // }
}

#[cfg(test)]
mod gen_sub_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT1: &'static str = "ALGVBMUCDTRXWPRNEBJHYRZGTALGFOIQSRK";
    const CIPHERTEXT2: &'static str =
        "🐏🚚📢🔝😩💡🆚🚅🍥☪🕳🚆🌃🍒🕳🚢🎴😩⏳👈📡🕳🕘📢☪🐏🚚📢😽⏯🚪😪💲🕳💮";

    #[test]
    fn encrypt_test1() {
        let mut cipher = GeneralSubstitution::default();
        cipher._assign_ct_alphabet("ODUSGPKLMECFJWRHVTYABZXNQI");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT1);
    }

    #[test]
    fn decrypt_test1() {
        let mut cipher = GeneralSubstitution::default();
        cipher._assign_ct_alphabet("ODUSGPKLMECFJWRHVTYABZXNQI");
        assert_eq!(cipher.decrypt(CIPHERTEXT1).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test2() {
        let mut cipher = GeneralSubstitution::default();
        cipher._assign_ct_alphabet("⏯🍥🆚💲📢🍒💮🚚💡🎴🚅😽⏳🌃🕳👈🔝☪📡🐏😩🕘🚆🚢😪🚪");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT2);
    }

    #[test]
    fn decrypt_test2() {
        let mut cipher = GeneralSubstitution::default();
        cipher._assign_ct_alphabet("⏯🍥🆚💲📢🍒💮🚚💡🎴🚅😽⏳🌃🕳👈🔝☪📡🐏😩🕘🚆🚢😪🚪");
        assert_eq!(cipher.decrypt(CIPHERTEXT2).unwrap(), PLAINTEXT);
    }
}
