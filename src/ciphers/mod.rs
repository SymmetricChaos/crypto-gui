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

use rand::prelude::ThreadRng;

pub trait Cipher {
    fn encrypt(&self, text: &str) -> Result<String,&'static str>;
    fn decrypt(&self, text: &str) -> Result<String,&'static str>;
    fn randomize(&mut self, rng: &mut ThreadRng);
    fn input_alphabet(&mut self) -> &mut String;
    fn output_alphabet(&mut self) -> &mut String;
}
