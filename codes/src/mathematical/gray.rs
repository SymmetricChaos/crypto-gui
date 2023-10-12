use utils::bits::IS_BITS;

use crate::{
    errors::CodeError,
    traits::{Code, IOMode, LetterAndWordCode},
};

pub struct GrayCode {
    pub maps: LetterAndWordCode<usize>,
    pub mode: IOMode,
    pub width: usize,
    pub fixed_width: bool,
}

impl Default for GrayCode {
    fn default() -> Self {
        let mut maps = LetterAndWordCode::default();
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        maps.set_letter_map(|(n, _)| n ^ (n >> 1));
        Self {
            width: 4,
            fixed_width: true,
            maps,
            mode: IOMode::Integer,
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

    pub fn decode_u32(&self, n: u32) -> String {
        let mut mask = n;
        let mut out = n;
        while mask != 0 {
            mask >>= 1;
            out ^= mask;
        }
        out.to_string()
    }

    pub fn set_letter_map(&mut self) {
        self.maps.set_letter_map(|(n, _)| n ^ (n >> 1))
    }

    pub fn set_word_map(&mut self) {
        self.maps.set_word_map(|(n, _)| n ^ (n >> 1))
    }
}

impl Code for GrayCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let m = 2_u32.pow(self.width as u32);
        let mut out = String::new();
        for s in text.split(" ") {
            let n = u32::from_str_radix(s, 10).map_err(|_| CodeError::invalid_input_group(s))?;
            if n >= m && self.fixed_width {
                return Err(CodeError::Input(format!(
                    "for a width of {} inputs must be less than {}",
                    self.width, m
                )));
            };
            out.push_str(&self.encode_u32(n));
            out.push(' ');
        }
        out.pop();
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();
        for s in text.split(" ") {
            if !IS_BITS.is_match(s) || (self.fixed_width && s.chars().count() != self.width) {
                return Err(CodeError::invalid_input_group(s));
            }
            let n = u32::from_str_radix(s, 2).map_err(|_| CodeError::invalid_input_group(s))?;
            out.push_str(&self.decode_u32(n));
            out.push(' ');
        }
        out.pop();
        Ok(out)
    }
}

#[cfg(test)]
mod gray_tests {
    use super::*;

    #[ignore]
    #[test]
    fn gray_code_generator() {
        let mut code = GrayCode::default();
        for n in 0..16 {
            println!("{}", code.encode_u32(n));
        }
        code.fixed_width = false;
        for n in [2,3,8,9,15,16] {
            println!("{}", code.encode_u32(n));
        }
    }

    const PLAINTEXT: &'static str = "1 2 3 14 15";
    const ENCODEDTEXT: &'static str = "0001 0011 0010 1001 1000";
    const ENCODEDTEXT_VAR: &'static str = "1 11 10 1001 1000";

    #[test]
    fn encode_test() {
        let code = GrayCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn encode_test_var() {
        let mut code = GrayCode::default();
        code.fixed_width = false;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_VAR);
    }

    #[test]
    fn decode_test() {
        let code = GrayCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }

    #[test]
    fn decode_test_var() {
        let mut code = GrayCode::default();
        code.fixed_width = false;
        assert_eq!(code.decode(ENCODEDTEXT_VAR).unwrap(), PLAINTEXT);
    }
}
