use itertools::Itertools;
use utils::{bits::IS_BITS, text_functions::string_chunks};

use crate::{
    errors::CodeError,
    letter_word_code::{IOMode, IntegerCodeMaps},
    traits::Code,
};

pub struct GrayCode {
    pub maps: IntegerCodeMaps,
    pub mode: IOMode,
    pub width: usize,
    pub fixed_width: bool,
    pub spaced: bool,
}

impl Default for GrayCode {
    fn default() -> Self {
        let mut maps = IntegerCodeMaps::new();
        maps.alphabet = String::from("ETAOINSHRDLCUMWFGYPBVKJXQZ");
        Self {
            width: 5,
            fixed_width: true,
            maps,
            mode: IOMode::Integer,
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
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let m = 2_u32.pow(self.width as u32);
        let mut out = String::new();

        if self.mode == IOMode::Letter {
            for c in text.chars() {
                let code = self.maps.char_to_int(c)? as u32;
                if code >= m && self.fixed_width {
                    return Err(CodeError::Input(format!(
                        "for a width of {} inputs must be less than {}",
                        self.width, m
                    )));
                };
                out.push_str(&self.encode_u32(code));
                if !self.fixed_width || self.spaced {
                    out.push(' ');
                }
            }
        } else if self.mode == IOMode::Word {
            for w in text.split(" ") {
                let code = self.maps.word_to_int(w)? as u32;
                if code >= m && self.fixed_width {
                    return Err(CodeError::Input(format!(
                        "for a width of {} inputs must be less than {}",
                        self.width, m
                    )));
                };
                out.push_str(&self.encode_u32(code));
                if !self.fixed_width || self.spaced {
                    out.push(' ');
                }
            }
        } else {
            for w in text.split(" ") {
                let n = u32::from_str_radix(w, 10).map_err(|e| CodeError::Input(e.to_string()))?;
                if n >= m && self.fixed_width {
                    return Err(CodeError::Input(format!(
                        "for a width of {} inputs must be less than {}",
                        self.width, m
                    )));
                };
                out.push_str(&self.encode_u32(n));
                if !self.fixed_width || self.spaced {
                    out.push(' ');
                }
            }
        }
        if !self.fixed_width || self.spaced {
            out.pop();
        };
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();
        let chunks = match self.fixed_width {
            true => string_chunks(&text.replace(" ", ""), self.width),
            false => text.split(" ").map(|st| st.to_string()).collect_vec(),
        };
        for s in chunks {
            if !IS_BITS.is_match(&s) || (self.fixed_width && s.chars().count() != self.width) {
                return Err(CodeError::invalid_input_group(&s));
            }
            let n = u32::from_str_radix(&s, 2).map_err(|_| CodeError::invalid_input_group(&s))?;
            let code = self.decode_to_u32(n);
            match self.mode {
                IOMode::Letter => {
                    out.push(self.maps.int_to_char(code as usize)?);
                }
                IOMode::Word => {
                    out.push_str(self.maps.int_to_word(code as usize)?);
                    out.push(' ');
                }
                IOMode::Integer => {
                    out.push_str(&code.to_string());
                    out.push(' ');
                }
            }
        }
        if out.chars().last() == Some(' ') {
            out.pop();
        }

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
        for n in [2, 3, 8, 9, 15, 16] {
            println!("{}", code.encode_u32(n));
        }
    }

    const PLAINTEXT_LTR: &'static str = "ETAOIN";
    const ENCODEDTEXT_LTR: &'static str = "000000000100011000100011000111";

    const PLAINTEXT: &'static str = "1 2 3 4 5 14 15";
    const ENCODEDTEXT: &'static str = "00001000110001000110001110100101000";
    const ENCODEDTEXT_SPACED: &'static str = "00001 00011 00010 00110 00111 01001 01000";
    const ENCODEDTEXT_VAR: &'static str = "1 11 10 110 111 1001 1000";

    #[test]
    fn encode_test() {
        let code = GrayCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn encode_test_ltr() {
        let mut code = GrayCode::default();
        code.mode = IOMode::Letter;
        assert_eq!(code.encode(PLAINTEXT_LTR).unwrap(), ENCODEDTEXT_LTR);
    }

    #[test]
    fn encode_test_var() {
        let mut code = GrayCode::default();
        code.fixed_width = false;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_VAR);
    }

    #[test]
    fn encode_test_spaced() {
        let mut code = GrayCode::default();
        code.spaced = true;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_SPACED);
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

    #[test]
    fn decode_test_spaced() {
        let code = GrayCode::default();
        assert_eq!(code.decode(ENCODEDTEXT_SPACED).unwrap(), PLAINTEXT);
    }

    #[test]
    fn decode_test_ltr() {
        let mut code = GrayCode::default();
        code.mode = IOMode::Letter;
        assert_eq!(code.decode(ENCODEDTEXT_LTR).unwrap(), PLAINTEXT_LTR);
    }
}
