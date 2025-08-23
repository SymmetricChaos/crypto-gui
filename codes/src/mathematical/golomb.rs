use super::{decode_prefix_to_strings, i32_to_u32_zigzag, string_to_i32s, string_to_u32s, swap_01};
use crate::{mathematical::truncated_binary::TruncatedBinary, traits::Code};
use num::Integer;
use utils::errors::GeneralError;

pub struct Golomb {
    pub spaced: bool,
    pub invert: bool,
    pub signed: bool,
    m: u32,
    rem_enconder: TruncatedBinary,
}

impl Default for Golomb {
    fn default() -> Self {
        Self {
            spaced: false,
            invert: false,
            signed: false,
            m: 3,
            rem_enconder: TruncatedBinary::new(3),
        }
    }
}

impl Golomb {
    pub fn set_modulus(&mut self, m: u32) {
        self.m = m;
        self.rem_enconder.set_consts(m);
    }

    pub fn u32_to_golomb(&self, x: u32) -> String {
        let (q, r) = x.div_rem(&self.m);
        // Encode the q portion in unary
        let mut out = "1".repeat(q as usize);
        out.push('0');

        // Encode the remainder with truncated binary
        out.push_str(&self.rem_enconder.u32_to_bits(r));
        if self.invert {
            out = swap_01(out);
        }
        out
    }

    pub fn i32_to_golomb(&self, x: i32) -> String {
        if let Some(n) = i32_to_u32_zigzag(x) {
            self.u32_to_golomb(n)
        } else {
            String::from("�")
        }
    }

    pub fn recognize_code(&self, text: &str) -> Vec<Option<u32>> {
        let mut out = Vec::new();

        let (z0, z1) = if self.invert { ('1', '0') } else { ('0', '1') };
        let mut mul: u32 = 0;
        let mut buffer = String::new();
        let mut rem = false;
        let mut n: u32 = 0;
        for bit in text.chars().filter(|c| *c == '0' || *c == '1') {
            if !rem {
                if bit == z1 {
                    mul += 1;
                }
                if bit == z0 {
                    n += self.m * mul;
                    rem = true;
                    continue;
                }
            }

            if rem {
                if bit == z0 {
                    buffer.push('0');
                } else {
                    buffer.push('1');
                }

                if buffer.len() > (self.m.ilog2() + 1) as usize {
                    out.push(None);
                    mul = 0;
                    buffer.clear();
                    rem = false;
                    n = 0;
                }
                if let Some(x) = self.rem_enconder.recognize_code(&buffer) {
                    out.push(Some(x + n));

                    mul = 0;
                    buffer.clear();
                    rem = false;
                    n = 0;
                }
            }
        }
        out
    }

    fn recognize_code_single(&self, text: &str) -> Option<u32> {
        let o = self.recognize_code(text);
        if o.len() != 1 {
            return None;
        } else {
            return o[0];
        }
    }
}

impl Code for Golomb {
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = Vec::new();

        if self.signed {
            for n in string_to_i32s(text, ",")? {
                out.push(self.i32_to_golomb(n))
            }
        } else {
            for n in string_to_u32s(text, ",")? {
                out.push(self.u32_to_golomb(n))
            }
        }

        if self.spaced {
            Ok(out.join(", "))
        } else {
            Ok(out.join(""))
        }
    }

    fn decode(&self, text: &str) -> Result<String, GeneralError> {
        let mut out: Vec<String> = Vec::new();

        if self.spaced {
            for section in text.split(",").map(|s| s.trim()) {
                decode_prefix_to_strings(
                    self.recognize_code_single(section),
                    self.signed,
                    &mut out,
                );
            }
        } else {
            for section in self.recognize_code(text) {
                decode_prefix_to_strings(section, self.signed, &mut out);
            }
        }

        Ok(out.join(", "))
    }
}

#[cfg(test)]
mod golomb_tests {
    use super::*;

    const PLAINTEXT: &'static str = "0, 1, 2, 3, 4, 5, 6, 7, 8, 9";
    const PLAINTEXT_SIGNED: &'static str = "0, -1, 1, -2, 2, -3, 3, -4, 4, -5";
    const ENCODEDTEXT: &'static str = "00010011100101010111100110101101111100";
    const ENCODEDTEXT_SP: &'static str = "00, 010, 011, 100, 1010, 1011, 1100, 11010, 11011, 11100";
    const ENCODEDTEXT_INV: &'static str = "11101100011010101000011001010010000011";
    const ENCODEDTEXT_INV_SP: &'static str =
        "11, 101, 100, 011, 0101, 0100, 0011, 00101, 00100, 00011";

    #[test]
    fn encode_test() {
        let mut code = Golomb::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
        code.spaced = true;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_SP);
        code.spaced = false;
        code.invert = true;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_INV);
        code.spaced = true;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_INV_SP);
        code.signed = true;
        code.spaced = false;
        code.invert = false;
        assert_eq!(code.encode(PLAINTEXT_SIGNED).unwrap(), ENCODEDTEXT);
        code.spaced = true;
        assert_eq!(code.encode(PLAINTEXT_SIGNED).unwrap(), ENCODEDTEXT_SP);
        code.spaced = false;
        code.invert = true;
        assert_eq!(code.encode(PLAINTEXT_SIGNED).unwrap(), ENCODEDTEXT_INV);
        code.spaced = true;
        assert_eq!(code.encode(PLAINTEXT_SIGNED).unwrap(), ENCODEDTEXT_INV_SP);
    }

    #[test]
    fn decode_test() {
        let mut code = Golomb::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
        code.spaced = true;
        assert_eq!(code.decode(ENCODEDTEXT_SP).unwrap(), PLAINTEXT);
        code.spaced = false;
        code.invert = true;
        assert_eq!(code.decode(ENCODEDTEXT_INV).unwrap(), PLAINTEXT);
        code.spaced = true;
        assert_eq!(code.decode(ENCODEDTEXT_INV_SP).unwrap(), PLAINTEXT);
        code.signed = true;
        code.spaced = false;
        code.invert = false;
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT_SIGNED);
        code.spaced = true;
        assert_eq!(code.decode(ENCODEDTEXT_SP).unwrap(), PLAINTEXT_SIGNED);
        code.spaced = false;
        code.invert = true;
        assert_eq!(code.decode(ENCODEDTEXT_INV).unwrap(), PLAINTEXT_SIGNED);
        code.spaced = true;
        assert_eq!(code.decode(ENCODEDTEXT_INV_SP).unwrap(), PLAINTEXT_SIGNED);
    }
}
