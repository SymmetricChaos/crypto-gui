use crate::{errors::CodeError, traits::Code};
use bimap::BiMap;
use lazy_static::lazy_static;
use utils::text_functions::bimap_from_iter;
// use regex::Regex;

lazy_static! {
    // pub static ref BIQUINARY: Regex = Regex::new(r"^([01])+$").unwrap();
    pub static ref BIQUINARY_MAP: BiMap<char, &'static str> =
        bimap_from_iter("0123456789".chars().zip(["01-00001",
        "01-00010",
        "01-00100",
        "01-01000",
        "01-10000",
        "10-00001",
        "10-00010",
        "10-00100",
        "10-01000",
        "10-10000"].into_iter()));
}
pub struct BiquinaryDecimal {}

impl Default for BiquinaryDecimal {
    fn default() -> Self {
        Self {}
    }
}

impl Code for BiquinaryDecimal {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
    }
}

#[cfg(test)]
mod balanced_ternary_tests {
    use super::*;

    const PLAINTEXT: &'static str = "";
    const ENCODEDTEXT: &'static str = "";

    #[test]
    fn encode_test() {
        let code = BiquinaryDecimal::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = BiquinaryDecimal::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
