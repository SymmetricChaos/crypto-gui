use super::string_to_i32s;
use crate::{errors::CodeError, traits::Code};
use itertools::Itertools;

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

pub fn recognize_code(text: &str, sep: &str) -> Vec<Option<i32>> {
    let mut output = Vec::new();

    for group in text.split(sep).filter(|s| !s.is_empty()) {
        output.push(decode_to_i32(group.trim()).ok());
    }

    output
}

pub fn encode_i32(n: i32) -> Result<String, CodeError> {
    if n == 0 {
        return Ok(String::from("0"));
    }
    let neg = n.is_negative();
    let mut n = n.abs();
    let mut output = String::new();

    while n != 0 {
        let mut rem = n % 3;
        n = n / 3;

        if rem == 2 {
            rem = -1;
            n += 1;
        }

        if rem == 0 {
            output.push('0')
        } else {
            if neg {
                if rem == 1 {
                    output.push('-')
                } else {
                    output.push('+')
                }
            } else {
                if rem == 1 {
                    output.push('+')
                } else {
                    output.push('-')
                }
            }
        }
    }
    Ok(output.chars().rev().collect())
}

pub struct BalancedTernary {}

impl Default for BalancedTernary {
    fn default() -> Self {
        Self {}
    }
}

impl BalancedTernary {}

impl Code for BalancedTernary {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = Vec::new();

        for n in string_to_i32s(text, ",")? {
            output.push(encode_i32(n)?);
        }

        Ok(output.into_iter().join(", "))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = Vec::new();

        for section in recognize_code(&text, ",") {
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
mod balanced_ternary_tests {
    use super::*;

    const PLAINTEXT: &'static str = "-3, -2, -1, 0, 1, 2, 3";
    const ENCODEDTEXT: &'static str = "-0, -+, -, 0, +, +-, +0";

    #[test]
    fn encode_test() {
        let code = BalancedTernary::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = BalancedTernary::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
