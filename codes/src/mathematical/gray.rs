use utils::bits::IS_BITS;

use crate::{errors::CodeError, traits::Code};

pub struct GrayCode {
    pub width: usize,
}

impl Default for GrayCode {
    fn default() -> Self {
        Self { width: 4 }
    }
}

impl GrayCode {
    pub fn encode_u32(&self, n: u32) -> String {
        let gray = n ^ (n >> 1);
        format!("{:0>1$b}", gray, self.width)
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

    // pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (u32, String)>> {
    //     let m = 2_u32.pow(self.width as u32);
    //     Box::new((0..m).map(|n| (n, self.encode_u32(n))))
    // }
}

impl Code for GrayCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let m = 2_u32.pow(self.width as u32);
        let mut out = String::new();
        for s in text.split(" ") {
            let n = u32::from_str_radix(s, 10).map_err(|_| CodeError::invalid_input_group(s))?;
            if n >= m {
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
            if !IS_BITS.is_match(s) || s.chars().count() != self.width {
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
        let code = GrayCode::default();
        for n in 0..16 {
            println!("{}", code.encode_u32(n));
        }
    }

    const PLAINTEXT: &'static str = "1 2 3 14 15";
    const ENCODEDTEXT: &'static str = "0001 0011 0010 1001 1000";

    #[test]
    fn encode_test() {
        let code = GrayCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let code = GrayCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
