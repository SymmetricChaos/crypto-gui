use utils::errors::GeneralError;

use super::{decode_prefix_to_strings, i32_to_u32_zigzag, string_to_i32s, string_to_u32s};
use crate::traits::Code;

pub struct UnaryCode {
    pub invert: bool,
    pub symmetric: bool,
    pub signed: bool,
    pub spaced: bool,
}

impl Default for UnaryCode {
    fn default() -> Self {
        UnaryCode {
            invert: false,
            symmetric: false,
            signed: false,
            spaced: false,
        }
    }
}

impl UnaryCode {
    pub fn encode_u32(&self, n: u32) -> String {
        let n = n as usize;
        if self.symmetric {
            if self.invert {
                if n == 0 {
                    return String::from("0");
                } else {
                    format!("1{}1", "0".repeat(n - 1))
                }
            } else {
                if n == 0 {
                    return String::from("1");
                } else {
                    format!("0{}0", "1".repeat(n - 1))
                }
            }
        } else {
            if self.invert {
                "0".repeat(n) + "1"
            } else {
                "1".repeat(n) + "0"
            }
        }
    }

    pub fn encode_i32(&self, n: i32) -> String {
        if let Some(x) = i32_to_u32_zigzag(n) {
            self.encode_u32(x)
        } else {
            String::from("�")
        }
    }

    pub fn recognize_code(&self, text: &str) -> Vec<Option<u32>> {
        let mut out = Vec::new();

        let mut ctr = 0;
        if self.symmetric {
            let (z0, z1) = if self.invert { ('1', '0') } else { ('0', '1') };

            for b in text.chars() {
                // Invalid characters immediatly give '?' response and restart
                if b != z0 && b != z1 {
                    out.push(None);
                    ctr = 0;
                    continue;
                }
                // The '1' bit on its own is a valid code
                if ctr == 0 && b == z1 {
                    out.push(Some(0));
                    continue;
                }
                // If the starting bit is '0' push it and continue
                if ctr == 0 && b == z0 {
                    ctr += 1;
                // Otherwise push the next bit on
                } else {
                    if b == z0 {
                        out.push(Some(ctr));
                        ctr = 0;
                    } else {
                        ctr += 1;
                    }
                }
            }
        } else {
            let (z0, z1) = if self.invert { ('0', '1') } else { ('1', '0') };
            for c in text.chars() {
                if c.is_whitespace() {
                    continue;
                }
                if c == z0 {
                    ctr += 1
                } else if c == z1 {
                    out.push(Some(ctr));
                    ctr = 0;
                } else {
                    out.push(None);
                    ctr = 0;
                }
            }
        }
        if ctr != 0 {
            out.push(None)
        }
        out
    }

    pub fn recognize_code_single(&self, text: &str) -> Option<u32> {
        let o = self.recognize_code(text);
        if o.len() != 1 {
            return None;
        } else {
            return o[0];
        }
    }
}

impl Code for UnaryCode {
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = Vec::new();

        if self.signed {
            for n in string_to_i32s(text, ",")? {
                out.push(self.encode_i32(n));
            }
        } else {
            for n in string_to_u32s(text, ",")? {
                out.push(self.encode_u32(n));
            }
        }

        if self.spaced {
            Ok(out.join(", "))
        } else {
            Ok(out.join(""))
        }
    }

    fn decode(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = Vec::new();

        if self.spaced {
            for section in text.split(",").map(|s| s.trim()) {
                decode_prefix_to_strings(
                    self.recognize_code_single(section),
                    self.signed,
                    &mut out,
                );
            }
        } else {
            for section in self.recognize_code(&text) {
                decode_prefix_to_strings(section, self.signed, &mut out);
            }
        }

        Ok(out.join(", "))
    }
}

#[cfg(test)]
mod unary_tests {
    use super::*;

    const PTEXT: &'static str = "0, 1, 2, 3, 4, 5, 6, 7, 8, 9";
    const PTEXT_SIGNED: &'static str = "0, -1, 1, -2, 2, -3, 3, -4, 4, -5";
    const ENCODEDTEXT: &'static str = "0101101110111101111101111110111111101111111101111111110";
    const ENCODEDTEXT_SYM: &'static str = "1000100110011100111100111110011111100111111100111111110";
    const ENCODEDTEXT_INV: &'static str = "1010010001000010000010000001000000010000000010000000001";
    const ENCODEDTEXT_INV_SYM: &'static str =
        "0111011001100011000011000001100000011000000011000000001";
    const ENCODEDTEXT_SP: &'static str =
        "0, 10, 110, 1110, 11110, 111110, 1111110, 11111110, 111111110, 1111111110";
    const ENCODEDTEXT_SP_SYM: &'static str =
        "1, 00, 010, 0110, 01110, 011110, 0111110, 01111110, 011111110, 0111111110";

    #[test]
    fn encode_test() {
        let mut code = UnaryCode::default();
        assert_eq!(code.encode(PTEXT).unwrap(), ENCODEDTEXT);
        code.symmetric = true;
        assert_eq!(code.encode(PTEXT).unwrap(), ENCODEDTEXT_SYM);
        code.symmetric = false;
        code.spaced = true;
        assert_eq!(code.encode(PTEXT).unwrap(), ENCODEDTEXT_SP);
        code.symmetric = true;
        assert_eq!(code.encode(PTEXT).unwrap(), ENCODEDTEXT_SP_SYM);
    }

    #[test]
    fn encode_test_signed() {
        let mut code = UnaryCode::default();
        code.signed = true;
        assert_eq!(code.encode(PTEXT_SIGNED).unwrap(), ENCODEDTEXT);
        code.symmetric = true;
        assert_eq!(code.encode(PTEXT_SIGNED).unwrap(), ENCODEDTEXT_SYM);
        code.symmetric = false;
        code.spaced = true;
        assert_eq!(code.encode(PTEXT_SIGNED).unwrap(), ENCODEDTEXT_SP);
        code.symmetric = true;
        assert_eq!(code.encode(PTEXT_SIGNED).unwrap(), ENCODEDTEXT_SP_SYM);
    }

    #[test]
    fn encode_test_inverted() {
        let mut code = UnaryCode::default();
        code.invert = true;
        assert_eq!(code.encode(PTEXT).unwrap(), ENCODEDTEXT_INV);
        code.symmetric = true;
        assert_eq!(code.encode(PTEXT).unwrap(), ENCODEDTEXT_INV_SYM);
    }

    #[test]
    fn decode_test() {
        let mut code = UnaryCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PTEXT);
        code.symmetric = true;
        assert_eq!(code.decode(ENCODEDTEXT_SYM).unwrap(), PTEXT);
        code.symmetric = false;
        code.spaced = true;
        assert_eq!(code.decode(ENCODEDTEXT_SP).unwrap(), PTEXT);
        code.symmetric = true;
        assert_eq!(code.decode(ENCODEDTEXT_SP_SYM).unwrap(), PTEXT);
    }

    #[test]
    fn decode_test_signed() {
        let mut code = UnaryCode::default();
        code.signed = true;
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PTEXT_SIGNED);
        code.symmetric = true;
        assert_eq!(code.decode(ENCODEDTEXT_SYM).unwrap(), PTEXT_SIGNED);
        code.symmetric = false;
        code.spaced = true;
        assert_eq!(code.decode(ENCODEDTEXT_SP).unwrap(), PTEXT_SIGNED);
        code.symmetric = true;
        assert_eq!(code.decode(ENCODEDTEXT_SP_SYM).unwrap(), PTEXT_SIGNED);
    }
}
