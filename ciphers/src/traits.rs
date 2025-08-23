use utils::errors::GeneralError;

pub trait Cipher {
    fn encrypt(&self, text: &str) -> Result<String, GeneralError>;
    fn decrypt(&self, text: &str) -> Result<String, GeneralError>;
}
