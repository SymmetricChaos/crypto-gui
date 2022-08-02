use crate::{
    ciphers::Cipher,
    errors::Error,
    global_rng::get_global_rng,
    text_aux::{shuffled_str, PresetAlphabet, VecString},
};

#[derive(Debug)]
pub struct GeneralSubstitution {
    pub pt_alphabet_string: String,
    pt_alphabet: VecString,
    pub ct_alphabet_string: String,
    ct_alphabet: VecString,
}

impl GeneralSubstitution {
    pub fn set_pt_alphabet(&mut self) {
        self.pt_alphabet = VecString::unique_from(&self.pt_alphabet_string);
    }

    pub fn set_ct_alphabet(&mut self) {
        self.ct_alphabet = VecString::unique_from(&self.ct_alphabet_string);
    }

    // easier fpr debugging
    pub fn _assign_pt_alphabet(&mut self, alphabet: &str) {
        self.pt_alphabet_string = alphabet.to_string();
        self.set_pt_alphabet();
    }

    pub fn _assign_ct_alphabet(&mut self, alphabet: &str) {
        self.ct_alphabet_string = alphabet.to_string();
        self.set_ct_alphabet();
    }

    pub fn encrypt_char(&self, c: char) -> char {
        let pos = self.pt_alphabet.get_pos_of(c).unwrap();
        self.ct_alphabet.get_char_at(pos).unwrap()
    }

    pub fn decrypt_char(&self, c: char) -> char {
        let pos = self.ct_alphabet.get_pos_of(c).unwrap();
        self.pt_alphabet.get_char_at(pos).unwrap()
    }

    fn validate_settings(&self) -> Result<(), Error> {
        if self.pt_alphabet.chars().count() != self.ct_alphabet.chars().count() {
            return Err(Error::key(
                "the input and output alphabets must have the same length",
            ));
        }
        Ok(())
    }

    fn validate_text_encrypt(&self, text: &str) -> Result<(), Error> {
        for c in text.chars() {
            if !self.pt_alphabet.contains(c) {
                return Err(Error::invalid_input_char(c));
            }
        }
        Ok(())
    }

    fn validate_text_decrypt(&self, text: &str) -> Result<(), Error> {
        for c in text.chars() {
            if !self.ct_alphabet.contains(c) {
                return Err(Error::invalid_input_char(c));
            }
        }
        Ok(())
    }
}

impl Default for GeneralSubstitution {
    fn default() -> Self {
        let pt_alphabet_string = String::from(PresetAlphabet::BasicLatin);
        let pt_alphabet = VecString::from(&pt_alphabet_string);
        let ct_alphabet_string = String::from("ZYXWVUTSRQPONMLKJIHGFEDCBA");
        let ct_alphabet = VecString::from(&ct_alphabet_string);
        Self {
            pt_alphabet_string,
            pt_alphabet,
            ct_alphabet_string,
            ct_alphabet,
        }
    }
}

impl Cipher for GeneralSubstitution {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        self.validate_settings()?;
        self.validate_text_encrypt(text)?;
        let out = text.chars().map(|c| self.encrypt_char(c)).collect();
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        self.validate_settings()?;
        self.validate_text_decrypt(text)?;
        let out = text.chars().map(|c| self.decrypt_char(c)).collect();
        Ok(out)
    }

    fn randomize(&mut self) {
        // keep the plaintext alphabet unchanged and make the ciphertext alphabet a shuffled version of it
        self.ct_alphabet_string = shuffled_str(&self.pt_alphabet_string, &mut get_global_rng());
        self.ct_alphabet = VecString::unique_from(&self.ct_alphabet_string);
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
