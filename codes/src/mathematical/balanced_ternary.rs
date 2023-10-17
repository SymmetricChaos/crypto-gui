use crate::{errors::CodeError, traits::Code};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref BALANCED_TERNARY: Regex = Regex::new(r"[-0\+]+").unwrap();
}

pub struct BaseN {}

impl Default for BaseN {
    fn default() -> Self {
        Self {}
    }
}

impl BaseN {
    pub fn encode_i32(n: i32) -> Result<String, CodeError> {
        todo!()
    }

    pub fn decode_to_i32(s: &str) -> Result<i32, CodeError> {
        let mut value = 0;
        let mut base = 1;
        for c in s.chars().rev() {
            value += match c {
                '+' => base,
                '-' => -base,
                '0' => 0,
                _ => return Err(CodeError::invalid_input_group(s)),
            };
            base *= 3;
        }
        Ok(value)
    }

    pub fn recognize_code(text: &str) -> Vec<Option<i32>> {
        let mut output = Vec::new();

        for cap in BALANCED_TERNARY.captures_iter(text) {
            let s = match cap.get(1) {
                Some(m) => m.as_str(),
                None => {
                    output.push(None);
                    continue;
                }
            };
            output.push(Some(Self::decode_to_i32(s).unwrap()));
        }

        output
    }
}

impl Code for BaseN {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = Vec::new();

        for group in text.split(" ") {
            if group.is_empty() {
                continue;
            }
            let n = i32::from_str_radix(group, 10)
                .map_err(|_| CodeError::invalid_input_group(group))?;
            output.push(Self::encode_i32(n)?);
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
mod base_n_tests {
    use super::*;

    const PLAINTEXT_INT: &'static str = "-3 -2 -1 0 1 2 3";
    const ENCODEDTEXT: &'static str = "";

    #[test]
    fn encode_test() {
        let code = BaseN::default();
        assert_eq!(code.encode(PLAINTEXT_INT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = BaseN::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT_INT);
    }
}
