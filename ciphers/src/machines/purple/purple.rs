use super::switch::Switches;
use crate::Cipher;
use std::{cell::LazyCell, collections::HashMap};
use utils::{errors::GeneralError, vecstring::VecString};

pub const PURPLE_ALPHABET: LazyCell<VecString> =
    LazyCell::new(|| VecString::from("AEIOUYBCDFGHJKLMNPQRSTVWXZ"));

pub struct Purple {
    pub switches: Switches, // this will be cloned during execution and then mutated
    plugboard: HashMap<char, usize>,
    plugboard_inv: HashMap<usize, char>,
    pub use_kana: bool,
}

impl Default for Purple {
    fn default() -> Self {
        let plugboard = HashMap::from([
            ('N', 0),
            ('O', 1),
            ('K', 2),
            ('T', 3),
            ('Y', 4),
            ('U', 5),
            ('X', 6),
            ('E', 7),
            ('Q', 8),
            ('L', 9),
            ('H', 10),
            ('B', 11),
            ('R', 12),
            ('M', 13),
            ('P', 14),
            ('D', 15),
            ('I', 16),
            ('C', 17),
            ('J', 18),
            ('A', 19),
            ('S', 20),
            ('V', 21),
            ('W', 22),
            ('G', 23),
            ('Z', 24),
            ('F', 25),
        ]);
        let plugboard_inv = HashMap::from(
            [
                ('N', 0),
                ('O', 1),
                ('K', 2),
                ('T', 3),
                ('Y', 4),
                ('U', 5),
                ('X', 6),
                ('E', 7),
                ('Q', 8),
                ('L', 9),
                ('H', 10),
                ('B', 11),
                ('R', 12),
                ('M', 13),
                ('P', 14),
                ('D', 15),
                ('I', 16),
                ('C', 17),
                ('J', 18),
                ('A', 19),
                ('S', 20),
                ('V', 21),
                ('W', 22),
                ('G', 23),
                ('Z', 24),
                ('F', 25),
            ]
            .map(|(a, b)| (b, a)),
        );
        Self {
            switches: Default::default(),
            plugboard,
            plugboard_inv,
            use_kana: false,
        }
    }
}

impl Purple {
    pub fn set_plugboard(&mut self, string: &str) -> Result<(), GeneralError> {
        if string.chars().count() != 26 {
            return Err(GeneralError::key(
                "plugboard must have exactly 26 characters",
            ));
        }
        self.plugboard.clear();
        self.plugboard_inv.clear();
        for (n, c) in string.chars().enumerate() {
            self.plugboard.insert(c, n);
            self.plugboard_inv.insert(n, c);
        }
        Ok(())
    }
}

impl Cipher for Purple {
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        // convert kana to romaji if needed
        // let text = if self.use_kana {
        //     let text = to_romaji(text, &NIHON_SHIKI);
        //     if let Err(e) = text {
        //         return Err(GeneralError::general(e.to_string()));
        //     }
        //     text.unwrap()
        // } else {
        //     text.to_string()
        // };

        // Clone switches then encrypt letters one by one, stepping each time
        let mut switches = self.switches.clone();
        let mut out = String::with_capacity(text.len());
        for c in text.chars() {
            let n = self
                .plugboard
                .get(&c)
                .ok_or(GeneralError::input("invalid character"))?;
            let encrypted = switches.encrypt_num(*n);
            out.push(
                *self
                    .plugboard_inv
                    .get(&encrypted)
                    .ok_or(GeneralError::input("invalid character"))?,
            );
            switches.step();
        }

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
        // convert kana to romaji if needed
        // let text = if self.use_kana {
        //     let text = to_romaji(text, &NIHON_SHIKI);
        //     if let Err(e) = text {
        //         return Err(GeneralError::general(e.to_string()));
        //     }
        //     text.unwrap()
        // } else {
        //     text.to_string()
        // };

        // Clone switches then decrypt letters one by one, stepping each time
        let mut switches = self.switches.clone();
        let mut out = String::with_capacity(text.len());
        for c in text.chars() {
            let n = self
                .plugboard
                .get(&c)
                .ok_or(GeneralError::input("invalid character"))?;
            let encrypted = switches.decrypt_num(*n);
            out.push(
                *self
                    .plugboard_inv
                    .get(&encrypted)
                    .ok_or(GeneralError::input("invalid character"))?,
            );
            switches.step();
        }

        Ok(out)
    }
}

#[cfg(test)]
mod purple_tests {

    use super::*;

    const PLAINTEXT: &'static str = "KONNICHIWAWATASHIWAAREKUSUDESU";
    const CIPHERTEXT: &'static str = "YTOUQBBXVSDRYJGSIVQIFHYNRTVMIT";

    #[test]
    fn encrypt() {
        let cipher = Purple::default();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt() {
        let cipher = Purple::default();
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
