use crate::{errors::CodeError, traits::Code};
use itertools::Itertools;
use utils::text_functions::num_to_digit;

use super::string_to_u32s;

pub struct BaseNBijective {
    pub radix: u32,
    pub little_endian: bool,
}

impl Default for BaseNBijective {
    fn default() -> Self {
        Self {
            radix: 2,
            little_endian: true,
        }
    }
}

impl BaseNBijective {
    pub fn validate(&self) -> Result<(), CodeError> {
        if self.radix < 1 || self.radix > 35 {
            return Err(CodeError::state(
                "bijective radix must be between 1 and 35, inclusive",
            ));
        }

        Ok(())
    }

    pub fn encode_u32(&self, n: u32) -> Result<String, CodeError> {
        if n == 0 {
            return Err(CodeError::input(
                "in bijective representation 0 is the empty string and cannot be represented",
            ));
        };

        if self.radix == 1 {
            return Ok("1".repeat(n as usize));
        }

        let mut out = Vec::with_capacity(32);
        let mut n = n;
        loop {
            let q = num::integer::div_ceil(n, self.radix) - 1;
            let a = n - q * self.radix;
            out.push(num_to_digit(a).expect("remainder should always be less than 35"));
            n = q;
            if n == 0 {
                break;
            }
        }
        if self.little_endian {
            Ok(out.iter().rev().collect())
        } else {
            Ok(out.iter().collect())
        }
    }

    pub fn decode_to_u32(&self, s: &str) -> Result<u32, CodeError> {
        let word: String = if self.little_endian {
            s.chars().collect()
        } else {
            s.chars().rev().collect()
        };

        let mut base = 1;
        let mut out = 0;
        for c in word.chars().rev() {
            let n = c
                .to_digit(36)
                .ok_or_else(|| CodeError::invalid_input_char(c))?;
            out += base * n;
            base *= self.radix;
        }
        Ok(out)
    }
}

impl Code for BaseNBijective {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        self.validate()?;
        let mut output = Vec::new();

        for n in string_to_u32s(text, ",")? {
            output.push(self.encode_u32(n)?);
        }

        Ok(output.into_iter().join(", "))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        self.validate()?;
        let mut output = Vec::new();

        for s in text.split(",").map(|s| s.trim()) {
            if s.is_empty() {
                continue;
            }
            output.push(self.decode_to_u32(s)?.to_string())
        }
        Ok(output.into_iter().join(", "))
    }
}

#[cfg(test)]
mod base_n_bijective_tests {
    use super::*;

    const PLAINTEXT: &'static str = "1, 2, 3, 4, 5, 6";
    const ENCODEDTEXT: &'static str = "1, 2, 11, 12, 21, 22";

    #[test]
    fn encode_test_bijective() {
        let code = BaseNBijective::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test_bijective() {
        let code = BaseNBijective::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
