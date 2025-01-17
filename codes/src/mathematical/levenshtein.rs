use crate::{errors::CodeError, traits::Code};

use super::{levenshtein_integers::LevenshteinCodeIntegers, string_to_u32s};

// https://en.wikipedia.org/wiki/Levenshtein_coding

pub struct LevenshteinCode {
    pub integer_code: LevenshteinCodeIntegers,
    pub spaced: bool,
}

impl Default for LevenshteinCode {
    fn default() -> Self {
        let codes = LevenshteinCodeIntegers::default();
        LevenshteinCode {
            integer_code: codes,
            spaced: false,
        }
    }
}

impl Code for LevenshteinCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = Vec::new();

        for n in string_to_u32s(text, ",")? {
            output.push(self.integer_code.encode_u32(n));
        }

        if self.spaced {
            Ok(output.join(", "))
        } else {
            Ok(output.join(""))
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let text = &text
            .chars()
            .filter(|c| *c == '0' || *c == '1')
            .collect::<String>();
        let mut output = Vec::new();

        for n in self.integer_code.decode_to_u32(text).into_iter() {
            if let Some(val) = n {
                output.push(val.to_string())
            } else {
                output.push(String::from("�"))
            }
        }

        Ok(output.join(", "))
    }
}

#[cfg(test)]
mod levenshtein_int_tests {
    use super::*;

    const PLAINTEXT_INT: &'static str = "0, 1, 2, 3";
    const ENCODEDTEXT: &'static str = "01011001101";
    const ENCODEDTEXT_SP: &'static str = "0, 10, 1100, 1101";

    #[test]
    fn encode_test() {
        let code = LevenshteinCode::default();
        assert_eq!(code.encode(PLAINTEXT_INT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn encode_sp_test() {
        let mut code = LevenshteinCode::default();
        code.spaced = true;
        assert_eq!(code.encode(PLAINTEXT_INT).unwrap(), ENCODEDTEXT_SP);
    }

    #[test]
    fn decode_test() {
        let code = LevenshteinCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT_INT);
    }

    #[test]
    fn decode_sp_test() {
        let code = LevenshteinCode::default();
        assert_eq!(code.decode(ENCODEDTEXT_SP).unwrap(), PLAINTEXT_INT);
    }
}
