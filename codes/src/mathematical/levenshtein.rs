use crate::{
    errors::CodeError,
    impl_code_for_integer_code,
    letter_word_code::{decode_or_err_char, IOMode, IntegerCodeMaps},
    traits::Code,
};

use super::levenshtein_integers::LevenshteinCodeIntegers;

// https://en.wikipedia.org/wiki/Levenshtein_coding

pub struct LevenshteinCode {
    pub maps: IntegerCodeMaps,
    pub mode: IOMode,
    pub integer_code: LevenshteinCodeIntegers,
    pub spaced: bool,
}

impl Default for LevenshteinCode {
    fn default() -> Self {
        let codes = LevenshteinCodeIntegers::default();
        let mut maps = IntegerCodeMaps::new();
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        LevenshteinCode {
            mode: IOMode::Integer,
            integer_code: codes,
            maps,
            spaced: false,
        }
    }
}

impl_code_for_integer_code!(LevenshteinCode);

#[cfg(test)]
mod levenshtein_int_tests {
    use super::*;

    const PLAINTEXT: &'static str = "ETAO";
    const PLAINTEXT_INT: &'static str = "0 1 2 3";
    const ENCODEDTEXT: &'static str = "01011001101";
    const ENCODEDTEXT_SPACED: &'static str = "0 10 1100 1101";

    #[test]
    fn encode_test() {
        let mut code = LevenshteinCode::default();

        assert_eq!(code.encode(PLAINTEXT_INT).unwrap(), ENCODEDTEXT);
        code.mode = IOMode::Letter;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
        code.spaced = true;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_SPACED);
    }

    #[test]
    fn decode_test() {
        let mut code = LevenshteinCode::default();
        code.mode = IOMode::Letter;
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
        assert_eq!(code.decode(ENCODEDTEXT_SPACED).unwrap(), PLAINTEXT);
    }
}
