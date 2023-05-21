pub mod chaocipher;
pub use chaocipher::Chaocipher;

pub mod hutton;
pub use hutton::{Hutton, HuttonVersion};

pub mod vigenere;
pub use vigenere::Vigenere;

pub mod bazeries;
pub use bazeries::Bazeries;

pub mod beaufort;
pub use beaufort::Beaufort;

pub mod quagmire;
pub use quagmire::{Quagmire, QuagmireVersion};

pub mod porta;
pub use porta::Porta;

pub mod alberti;
pub use alberti::Alberti;

pub mod m94;
pub use m94::M94;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PolyMode {
    CylicKey,
    Autokey,
    ProgKey,
}
