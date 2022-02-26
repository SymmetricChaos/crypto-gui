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

pub mod playfair;
pub use playfair::Playfair;

pub mod slidefair;
pub use slidefair::Slidefair;

pub mod alberti_disk;
pub use alberti_disk::Alberti;

pub mod polybius;
pub use polybius::Polybius;

pub mod m94;
pub use m94::M94;

pub mod vigenere;
pub use vigenere::Vigenere;

pub mod beaufort;
pub use beaufort::Beaufort;

pub mod columnar;
pub use columnar::Columnar;

pub mod adfgvx;
pub use adfgvx::ADFGVX;

pub mod b64;
pub use b64::B64;

use rand::prelude::ThreadRng;
use crate::errors::CipherError;

pub trait Cipher {
    fn encrypt(&self, text: &str) -> Result<String,CipherError>;
    fn decrypt(&self, text: &str) -> Result<String,CipherError>;
    fn randomize(&mut self, rng: &mut ThreadRng);
    fn reset(&mut self);
    fn get_input_alphabet(&self) -> &String;
    fn get_mut_input_alphabet(&mut self) -> &mut String;
    fn validate_settings(&self) -> Result<(),CipherError>;
}



#[derive(Debug,Copy,Clone,PartialEq, Eq)]
pub enum PolyMode {
    CylicKey,
    Autokey,
    ProgKey
}