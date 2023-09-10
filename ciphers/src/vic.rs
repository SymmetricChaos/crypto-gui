use crate::Cipher;

pub struct Vic {
    alphabet: VecString,
    phrase: String,
    date: String,
    pin: String,
    keygroup: String,
}

impl Vic {
    pub fn key_derivation_string(&self) -> String {}

    pub fn key_derivation(&self) {}
}

impl Cipher for Vic {
    fn encrypt(&self, text: &str) -> Result<String, crate::CipherError> {
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, crate::CipherError> {
        todo!()
    }
}
