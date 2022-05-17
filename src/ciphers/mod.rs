pub mod polybius;
pub mod tactical;

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

pub mod seriated_playfair;

pub mod alberti_disk;
pub use alberti_disk::Alberti;

pub mod m94;
pub use m94::M94;

pub mod vigenere;
pub use vigenere::Vigenere;

pub mod beaufort;
pub use beaufort::Beaufort;

pub mod columnar;
pub use columnar::Columnar;

pub mod enigma;
pub use enigma::EnigmaM3;

pub mod grille;
pub use grille::Grille;

pub mod sigaba;
pub use sigaba::Sigaba;

pub mod bazeries;
pub use bazeries::Bazeries;

pub mod chaocipher;
pub use chaocipher::Chaocipher;

pub mod syctale;
pub use syctale::Scytale;

pub mod rail_fence;
pub use rail_fence::RailFence;

pub mod porta;
pub use porta::Porta;

pub mod hebern;
pub use hebern::HebernRotor;

pub mod two_square;
pub use two_square::TwoSquare;

pub mod four_square;
pub use four_square::FourSquare;

pub mod hutton;
pub use hutton::Hutton;

// pub mod route;
// pub use route::Route;

use crate::errors::CipherError;
use rand::prelude::StdRng;

pub trait Cipher {
    fn encrypt(&self, text: &str) -> Result<String, CipherError>;
    fn decrypt(&self, text: &str) -> Result<String, CipherError>;
    fn randomize(&mut self, rng: &mut StdRng);
    fn reset(&mut self);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PolyMode {
    CylicKey,
    Autokey,
    ProgKey,
}
