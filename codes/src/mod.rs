pub mod errors;
pub use errors::CodeError;

pub mod traits;

pub mod binary_to_text;
pub use binary_to_text::base32::Base32;
pub use binary_to_text::base64::Base64;
pub use binary_to_text::pgp_words::PgpWords;
pub use binary_to_text::skey::SKeyWords;

pub mod ecc;
pub use ecc::hamming::HammingCode;
pub use ecc::isbn::Isbn;
pub use ecc::luhn::LuhnAlgorithm;
pub use ecc::m_of_n::MofNCode;
pub use ecc::parity_check::ParityBit;
pub use ecc::repetition::Repetition;

pub mod ascii;
pub use ascii::Ascii;

pub mod godel;
pub use godel::Godel;

pub mod fibonacci;
pub use fibonacci::FibonacciCode;
pub mod fibonacci_integers;
pub use fibonacci_integers::FibonacciCodeIntegers;

pub mod levenshtein;
pub use levenshtein::LevenshteinCode;
pub mod levenshtein_integers;
pub use levenshtein_integers::LevenshteinCodeIntegers;

// pub mod elias;
// pub use elias::EliasCode;
// pub mod elias_integers;
// pub use elias_integers::EliasCodeIntegers;

pub mod unary;
pub use unary::UnaryCode;

pub mod spelling_alphabet;
pub use spelling_alphabet::SpellingAlphabet;

pub mod unicode;
pub use unicode::Unicode;

pub mod baudot;
pub use baudot::Baudot;

pub mod bacon;
pub use bacon::Bacon;

pub mod punycode;
pub use punycode::Punycode;

pub mod block;
pub use block::BlockCode;

pub mod morse;
pub use morse::Morse;
pub mod morse_encodings;

pub mod tap_code;
pub use tap_code::TapCode;

pub mod needle;
pub use needle::Needle;

pub mod romaji;
pub use romaji::romaji::Romaji;

pub mod linotype;
pub use linotype::Linotype;
