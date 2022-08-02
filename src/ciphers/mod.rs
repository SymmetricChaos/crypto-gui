pub mod hebern;
pub mod playfair;
pub mod polyalphabetic;
pub mod polybius;
pub mod substitution;
pub mod tactical;
pub mod transposition;

pub mod m209;
pub use m209::M209;

pub mod enigma;
pub use enigma::EnigmaM3;

pub mod sigaba;
pub use sigaba::Sigaba;

pub mod purple;
pub use purple::Purple;

// pub mod route;
// pub use route::Route;

use crate::errors::Error;

pub trait Cipher {
    fn encrypt(&self, text: &str) -> Result<String, Error>;
    fn decrypt(&self, text: &str) -> Result<String, Error>;
    fn randomize(&mut self);
    fn reset(&mut self);
}
