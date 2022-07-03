pub mod playfair;
pub mod polyalphabetic;
pub mod polybius;
pub mod substitution;
pub mod tactical;
pub mod transposition;
pub mod hebern;

pub mod m209;
pub use m209::M209;

pub mod m94;
pub use m94::M94;

pub mod enigma;
pub use enigma::EnigmaM3;

pub mod sigaba;
pub use sigaba::Sigaba;

pub mod purple;

// pub mod route;
// pub use route::Route;

use crate::errors::CipherError;

pub trait Cipher {
    fn encrypt(&self, text: &str) -> Result<String, CipherError>;
    fn decrypt(&self, text: &str) -> Result<String, CipherError>;
    fn randomize(&mut self);
    fn reset(&mut self);
}
