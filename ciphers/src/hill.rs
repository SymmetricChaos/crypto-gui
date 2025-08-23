use utils::vecstring::VecString;

use crate::{Cipher, GeneralError};

// https://patents.google.com/patent/US1845947

pub struct Hill {
    pub alphabet: VecString,
    // pub mat: SomeKindOfMatrix, // some matrix where we can calculate the modular matrix inverse
    pub key1: String,
    pub key2: String,
}

impl Hill {}

impl Cipher for Hill {
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        // Vigenere step?
        // Matrix step
        // Vigenere step?
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
        todo!()
    }
}

// #[cfg(test)]
// mod hill_test {

//     use super::*;

// }
