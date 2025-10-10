use crate::traits::Code;
use itertools::Itertools;
use utils::{bits::IS_BITS, errors::GeneralError, text_functions::string_chunks};

pub struct GrayCode {
    pub width: usize,
    pub fixed_width: bool,
    pub spaced: bool,
}

impl Default for GrayCode {
    fn default() -> Self {
        Self {
            width: 5,
            fixed_width: true,
            spaced: false,
        }
    }
}

impl GrayCode {
    pub fn encode_u32(&self, n: u32) -> String {
        let gray = n ^ (n >> 1);
        if self.fixed_width {
            format!("{:0>1$b}", gray, self.width)
        } else {
            format!("{:b}", gray)
        }
    }

    pub fn decode_to_u32(&self, n: u32) -> u32 {
        let mut mask = n;
        let mut out = n;
        while mask != 0 {
            mask >>= 1;
            out ^= mask;
        }
        out
    }
}

impl Code for GrayCode {
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
        let m = 2_u32.pow(self.width as u32);
        let mut out = Vec::new();

        for w in text.split(",") {
            let n = u32::from_str_radix(w.trim(), 10)
                .map_err(|e| GeneralError::input(e.to_string()))?;
            if n >= m && self.fixed_width {
                return Err(GeneralError::input(format!(
                    "for a width of {} inputs must be less than {}",
                    self.width, m
                )));
            };
            out.push(self.encode_u32(n).clone());
        }

        if !self.fixed_width || self.spaced {
            Ok(out.join(", "))
        } else {
            Ok(out.join(""))
        }
    }

    fn decode(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = Vec::new();
        let chunks = match self.fixed_width {
            true => string_chunks(
                &text
                    .chars()
                    .filter(|c| *c == '0' || *c == '1')
                    .collect::<String>(),
                self.width,
            ),
            false => text
                .split(",")
                .map(|st| st.trim().to_string())
                .collect_vec(),
        };
        for s in chunks {
            if !IS_BITS.is_match(&s) || (self.fixed_width && s.chars().count() != self.width) {
                return Err(GeneralError::invalid_input_group(&s));
            }
            let n =
                u32::from_str_radix(&s, 2).map_err(|_| GeneralError::invalid_input_group(&s))?;
            let code = self.decode_to_u32(n);

            out.push(code.to_string());
        }

        Ok(out.join(", "))
    }
}

#[cfg(test)]
mod gray_tests {
    use super::*;

    const PTEXT: &'static str = "1, 2, 3, 4, 5, 14, 15";
    const ENCODEDTEXT: &'static str = "00001000110001000110001110100101000";
    const ENCODEDTEXT_SPACED: &'static str = "00001, 00011, 00010, 00110, 00111, 01001, 01000";
    const ENCODEDTEXT_VAR: &'static str = "1, 11, 10, 110, 111, 1001, 1000";

    #[test]
    fn encode_test() {
        let code = GrayCode::default();
        assert_eq!(code.encode(PTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn encode_test_var() {
        let mut code = GrayCode::default();
        code.fixed_width = false;
        assert_eq!(code.encode(PTEXT).unwrap(), ENCODEDTEXT_VAR);
    }

    #[test]
    fn encode_test_spaced() {
        let mut code = GrayCode::default();
        code.spaced = true;
        assert_eq!(code.encode(PTEXT).unwrap(), ENCODEDTEXT_SPACED);
    }

    #[test]
    fn decode_test() {
        let code = GrayCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PTEXT);
    }

    #[test]
    fn decode_test_var() {
        let mut code = GrayCode::default();
        code.fixed_width = false;
        assert_eq!(code.decode(ENCODEDTEXT_VAR).unwrap(), PTEXT);
    }

    #[test]
    fn decode_test_spaced() {
        let code = GrayCode::default();
        assert_eq!(code.decode(ENCODEDTEXT_SPACED).unwrap(), PTEXT);
    }
}
