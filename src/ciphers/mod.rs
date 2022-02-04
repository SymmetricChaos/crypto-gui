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

pub mod cyclic_key;
pub use cyclic_key::CyclicKey;

pub mod autokey;
pub use autokey::Autokey;

pub mod progressive_key;
pub use progressive_key::ProgressiveKey;

pub mod alberti_disk;
pub use alberti_disk::Alberti;

pub mod composite;

use rand::prelude::ThreadRng;
use crate::errors::{CipherError, CipherErrors};

pub trait Cipher {
    fn encrypt(&self, text: &str) -> Result<String,CipherError>;
    fn decrypt(&self, text: &str) -> Result<String,CipherError>;
    fn randomize(&mut self, rng: &mut ThreadRng);
    fn get_mut_input_alphabet(&mut self) -> &mut String;
    fn get_mut_output_alphabet(&mut self) -> &mut String;
    fn get_input_alphabet(&mut self) -> &String;
    fn get_output_alphabet(&mut self) -> &String;
    fn validate_settings(&self) -> Result<(),CipherErrors>;
    //fn get_key_state(&self) -> some complex thing? idk or this could be optional
}


#[derive(Debug,Copy,Clone,PartialEq, Eq)]
pub enum PolyMode {
    Vigenere,
    Beaufort,
}