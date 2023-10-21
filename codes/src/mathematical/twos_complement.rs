use crate::{errors::CodeError, traits::Code};
use utils::text_functions::string_chunks;

pub struct TwosComplement {}

impl Default for TwosComplement {
    fn default() -> Self {
        Self {}
    }
}

impl TwosComplement {
    pub fn encode_i32(n: i32) -> String {
        format!("{n:0>32b}")
    }

    pub fn decode_to_i32(s: &str) -> Result<i32, CodeError> {
        if s.len() != 32 || !s.chars().all(|c| c == '1' || c == '0') {
            return Err(CodeError::invalid_input_group(s));
        }
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

        for group in string_chunks(text, 32) {
            output.push(Self::decode_to_i32(&group).ok());
        }

        output
    }
}

impl Code for TwosComplement {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = String::new();

        for group in text.split(" ") {
            if group.is_empty() {
                continue;
            }
            let n = i32::from_str_radix(group, 10)
                .map_err(|_| CodeError::invalid_input_group(group))?;
            output.push_str(&Self::encode_i32(n));
        }

        Ok(output)
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
mod twos_complement_tests {
    use super::*;

    const PLAINTEXT: &'static str = "-3 -2 -1 0 1 2 3";
    const ENCODEDTEXT: &'static str = "11111111111111111111111111111101111111111111111111111111111111101111111111111111111111111111111100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000001000000000000000000000000000000011";

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
