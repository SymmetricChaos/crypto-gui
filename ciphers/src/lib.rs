pub mod digital;
pub mod machines;
pub mod playfair;
pub mod polyalphabetic;
pub mod polybius;
pub mod solitaire;
pub mod substitution;
pub mod tactical;
pub mod transposition;
pub mod vic;

pub mod errors;
pub use errors::CipherError;
pub mod ids;
pub mod rotors;
pub mod traits;
pub use traits::Cipher;

#[macro_export]
macro_rules! lazy_regex {
    ($($name: ident, $regex: literal);+ $(;)?) => {
        $(
        pub const $name: std::cell::LazyCell<regex::Regex> =
            std::cell::LazyCell::new(|| regex::Regex::new($regex).unwrap());
        )+
    };
}
