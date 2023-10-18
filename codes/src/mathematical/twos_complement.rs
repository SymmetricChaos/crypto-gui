use crate::{errors::CodeError, traits::Code};
use itertools::Itertools;

pub struct TwosComplement {}

impl Default for TwosComplement {
    fn default() -> Self {
        Self {}
    }
}

impl TwosComplement {
    pub fn decode_to_i32(s: &str) -> Result<i32, CodeError> {
        let mut bits = s.chars();
        let mut out = if let Some(c) = bits.next() {
            match c {
                '0' => 0,
                '1' => i32::MIN,
                _ => return Err(CodeError::invalid_input_group(s)),
            }
        } else {
            return Err(CodeError::invalid_input_group(s));
        };
        out += i32::from_str_radix(&bits.collect::<String>(), 2)
            .map_err(|e| CodeError::Input(e.to_string()))?;
        Ok(out)
    }

    pub fn recognize_code(text: &str) -> Vec<Option<i32>> {
        let mut output = Vec::new();

        for group in text.split(" ").filter(|s| !s.is_empty()) {
            output.push(Self::decode_to_i32(group).ok());
        }

        output
    }
}

impl Code for TwosComplement {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = Vec::new();

        for group in text.split(" ") {
            if group.is_empty() {
                continue;
            }
            let n = i32::from_str_radix(group, 10)
                .map_err(|_| CodeError::invalid_input_group(group))?;
            output.push(format!("{n:0>32b}"));
        }

        Ok(output.into_iter().join(" "))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = String::new();

        for section in Self::recognize_code(&text) {
            if let Some(code) = section {
                output.push_str(&code.to_string());
                output.push(' ');
            } else {
                output.push_str("� ");
            }
        }
        output.pop();

        Ok(output)
    }
}

#[cfg(test)]
mod balanced_ternary_tests {
    use super::*;

    const PLAINTEXT: &'static str = "-3 -2 -1 0 1 2 3";
    const ENCODEDTEXT: &'static str = "11111111111111111111111111111101 11111111111111111111111111111110 11111111111111111111111111111111 00000000000000000000000000000000 00000000000000000000000000000001 00000000000000000000000000000010 00000000000000000000000000000011";

    #[test]
    #[ignore]
    fn check_fmt() {
        for i in [-3, -2, -1, 0, 1, 2, 3] {
            println!("{i:0>32b}")
        }
    }

    #[test]
    fn encode_test() {
        let code = TwosComplement::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = TwosComplement::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
