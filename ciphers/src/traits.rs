use crate::errors::CipherError;

pub trait Cipher {
    fn encrypt(&self, text: &str) -> Result<String, CipherError>;
    fn decrypt(&self, text: &str) -> Result<String, CipherError>;
}
