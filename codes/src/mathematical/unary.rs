use itertools::Itertools;

use crate::{errors::CodeError, traits::Code};

use super::string_to_usizes;

pub struct UnaryCode {
    pub invert: bool,
    pub spaced: bool,
}

impl Default for UnaryCode {
    fn default() -> Self {
        UnaryCode {
            invert: false,
            spaced: false,
        }
    }
}

impl UnaryCode {
    pub fn encode_usize(&self, n: usize) -> String {
        if self.invert {
            "0".repeat(n) + "1"
        } else {
            "1".repeat(n) + "0"
        }
    }

    pub fn recognize_code(&self, text: &str) -> Vec<Option<usize>> {
        let mut output = Vec::new();

        let (z0, z1) = if self.invert { ('0', '1') } else { ('1', '0') };

        let mut ctr = 0;
        for c in text.chars() {
            if c.is_whitespace() {
                continue;
            }
            if c == z0 {
                ctr += 1
            } else if c == z1 {
                output.push(Some(ctr));
                ctr = 0;
            } else {
                output.push(None);
                ctr = 0;
            }
        }
        if ctr != 0 {
            output.push(None)
        }
        output
    }
}

impl Code for UnaryCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = Vec::new();

        for n in string_to_usizes(text, ",")? {
            output.push(self.encode_usize(n));
        }

        if self.spaced {
            Ok(output.into_iter().join(", "))
        } else {
            Ok(output.into_iter().join(""))
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = Vec::new();

        for section in self.recognize_code(&text) {
            if let Some(code) = section {
                output.push(code.to_string());
            } else {
                output.push(String::from("�"));
            }
        }

        Ok(output.into_iter().join(", "))
    }
}

#[cfg(test)]
mod unary_tests {
    use super::*;

    const PLAINTEXT: &'static str = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17";
    const ENCODEDTEXT: &'static str = "10110111011110111110111111011111110111111110111111111011111111110111111111110111111111111011111111111110111111111111110111111111111111011111111111111110111111111111111110";

    #[test]
    fn encode_test() {
        let code = UnaryCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = UnaryCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
