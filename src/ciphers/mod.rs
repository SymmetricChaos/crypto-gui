pub mod caesar;
pub use caesar::Caesar;

pub mod affine;
pub use affine::Affine;

pub mod substitution;
pub use substitution::GeneralSubstitution;

pub mod decoder_ring;
pub use decoder_ring::DecoderRing;

pub mod m209;
pub use m209::M209;

pub mod vigenere;
pub use vigenere::Vigenere;

pub mod beaufort;
pub use beaufort::Beaufort;

pub mod playfair;
pub use playfair::Playfair;

use rand::prelude::ThreadRng;
use crate::errors::CipherError;

pub trait Cipher {
    fn encrypt(&self, text: &str) -> Result<String,CipherError>;
    fn decrypt(&self, text: &str) -> Result<String,CipherError>;
    fn randomize(&mut self, rng: &mut ThreadRng);
    fn input_alphabet(&mut self) -> &mut String;
    fn output_alphabet(&mut self) -> &mut String;
    //fn validate_settings(&self) -> Result<(),CipherError>;
}

#[derive(Debug,Copy,Clone,PartialEq, Eq)]
pub enum PolyalphabeticMode {
    Cyclic,
    Autokey,
    Progressive,
}