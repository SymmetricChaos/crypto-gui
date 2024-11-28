use crate::{errors::CodeError, traits::Code};
use itertools::Itertools;
use num::{Integer, Zero};
use utils::text_functions::num_to_digit;

pub struct BaseN {
    pub radix: i32,
    pub little_endian: bool,
}

impl Default for BaseN {
    fn default() -> Self {
        Self {
            radix: -2,
            little_endian: true,
        }
    }
}

impl BaseN {
    pub fn validate(&self) -> Result<(), CodeError> {
        if self.radix > -2 || self.radix < -36 {
            return Err(CodeError::state(
                "radix must be between -2 and -36, inclusive",
            ));
        }
        Ok(())
    }

    pub fn encode_i32(&self, n: i32) -> Result<String, CodeError> {
        if n.is_zero() {
            return Ok(String::from("0"));
        }
        let mut n = n;
        let mut s = Vec::new();
        while n != 0 {
            let (q, r) = n.div_rem(&self.radix);
            s.push(num_to_digit(-r as u32).expect("remainder should always be less than 36"));

            n = q;
        }
        if self.little_endian {
            Ok(s.iter().rev().collect())
        } else {
            Ok(s.iter().collect())
        }
    }

    pub fn decode_to_i32(&self, s: &str) -> Result<i32, CodeError> {
        let word: String = if self.little_endian {
            s.chars().collect()
        } else {
            s.chars().rev().collect()
        };

        i32::from_str_radix(&word, -self.radix as u32).map_err(|e| CodeError::Input(e.to_string()))
    }
}

impl Code for BaseN {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        self.validate()?;
        let mut output = Vec::new();

        for group in text.split(" ") {
            if group.is_empty() {
                continue;
            }
            let n = i32::from_str_radix(group, 10)
                .map_err(|_| CodeError::invalid_input_group(group))?;
            output.push(self.encode_i32(n)?);
        }

        Ok(output.into_iter().join(" "))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        self.validate()?;
        let mut output = String::new();

        for s in text.split(" ") {
            if s.is_empty() {
                continue;
            }
            output.push_str(&format!("{} ", self.decode_to_i32(s)?))
        }
        output.pop();

        Ok(output)
    }
}

#[cfg(test)]
mod negative_base_n_tests {
    use super::*;

    const PLAINTEXT_INT: &'static str = "0 1 2 3 4 5";
    const ENCODEDTEXT: &'static str = "0 1 10 11 100 101";

    const PLAINTEXT_INT_BE: &'static str = "0 1 2 3 4 5";
    const ENCODEDTEXT_BE: &'static str = "0 1 01 11 001 101";
}
