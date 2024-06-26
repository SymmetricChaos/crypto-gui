use utils::vecstring::VecString;

use crate::{Cipher, CipherError};

// https://patents.google.com/patent/US1845947

pub struct Hill {
    pub alphabet: VecString,
    // pub mat: SomeKindOfMatrix, // some matrix where we can calculate the modular matrix inverse
    pub key1: String,
    pub key2: String,
}

impl Hill {}

impl Cipher for Hill {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        // Vigenere step?
        // Matrix step
        // Vigenere step?
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }
}

// #[cfg(test)]
// mod hill_test {

//     use super::*;

// }
