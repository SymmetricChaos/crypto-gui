use crate::{errors::CodeError, mathematical::truncated_binary::TruncatedBinary, traits::Code};
use num::Integer;
use utils::bits::bits_from_str;

use super::string_to_u32s;

pub struct Golomb {
    m: u32,
    spaced: bool,
}

impl Default for Golomb {
    fn default() -> Self {
        Self {
            m: 3,
            spaced: false,
        }
    }
}

impl Golomb {
    pub fn u32_to_bits(&self, x: u32) -> String {
        let (q, r) = x.div_rem(&self.m);
        // Encode the q portion in unary
        let mut out = "1".repeat(q as usize);
        out.push('0');

        // Encode the remainder with truncated binary
        let b = self.m.ilog2();
        out.push_str(&TruncatedBinary::new(self.m).u32_to_bits(r));
        out
    }

    pub fn bits_to_u32(&self, x: String) -> Vec<u32> {
        todo!()
    }
}

impl Code for Golomb {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::new();

        for x in string_to_u32s(text, ",")? {
            out.push(self.u32_to_bits(x))
        }

        if self.spaced {
            Ok(out.join(", "))
        } else {
            Ok(out.join(""))
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
    }
}

#[cfg(test)]
mod golomb_tests {
    use super::*;

    const PLAINTEXT: &'static str = "0, 1, 2, 3, 4, 5, 6, 7, 8";
    const ENCODEDTEXT: &'static str = "000100111001010101111001101011011";

    #[test]
    fn encode_test() {
        let code = Golomb::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decode_test() {
        let mut code = Golomb::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
