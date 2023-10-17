use crate::{errors::CodeError, traits::Code};
use itertools::Itertools;

pub struct BalancedTernary {}

impl Default for BalancedTernary {
    fn default() -> Self {
        Self {}
    }
}

impl BalancedTernary {
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

        for group in text.split(" ").filter(|s| !s.is_empty()) {
            output.push(Self::decode_to_i32(group).ok());
        }

        output
    }
}

impl Code for BalancedTernary {
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
mod balanced_ternary_tests {
    use super::*;

    const PLAINTEXT: &'static str = "-3 -2 -1 0 1 2 3";
    const ENCODEDTEXT: &'static str = "-0 -+ - 0 + +- +0";

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
