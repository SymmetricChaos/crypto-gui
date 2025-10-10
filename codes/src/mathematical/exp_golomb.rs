use super::{decode_prefix_to_strings, i32_to_u32_zigzag, string_to_i32s, string_to_u32s, swap_01};
use crate::traits::Code;
use itertools::Itertools;
use std::u32;
use utils::errors::GeneralError;

pub fn u32_to_exp_golomb(n: u32) -> String {
    if n == u32::MAX {
        String::from("00000000000000000000000000000000100000000000000000000000000000000")
    } else {
        let s = format!("{:b}", n + 1);
        let mut z = "0".repeat(s.len() - 1);
        z.push_str(&s);
        z
    }
}

pub fn i32_to_exp_golomb(n: i32) -> String {
    if let Some(x) = i32_to_u32_zigzag(n) {
        u32_to_exp_golomb(x)
    } else {
        String::from("�")
    }
}

pub struct ExpGolomb {
    pub spaced: bool,
    pub invert: bool,
    pub signed: bool,
}

impl Default for ExpGolomb {
    fn default() -> Self {
        Self {
            spaced: false,
            invert: false,
            signed: false,
        }
    }
}

impl ExpGolomb {
    pub fn recognize_code(&self, s: &str) -> Vec<Option<u32>> {
        let mut out = Vec::new();
        let mut ctr = 0;
        let mut counting_up = true;
        let mut buffer = String::new();
        let (z0, z1) = if self.invert { ('1', '0') } else { ('0', '1') };
        for b in s.chars().filter(|c| *c == z0 || *c == z1) {
            if counting_up {
                if b == z0 {
                    ctr += 1;
                }
                if b == z1 {
                    if ctr == 0 {
                        out.push(Some(0));
                        continue;
                    }
                    buffer.push(b);
                    counting_up = false;
                }
            } else {
                buffer.push(b);
                ctr -= 1;
                if ctr <= 0 {
                    if self.invert {
                        buffer = swap_01(buffer);
                    }

                    if buffer == "100000000000000000000000000000000" {
                        out.push(Some(u32::MAX));
                    } else {
                        if let Ok(n) = u32::from_str_radix(&buffer, 2) {
                            out.push(Some(n - 1));
                        } else {
                            out.push(None);
                        }
                    }
                    counting_up = true;
                    buffer.clear();
                    ctr = 0;
                    continue;
                }
            }
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

impl Code for ExpGolomb {
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = Vec::new();
        if self.signed {
            for n in string_to_i32s(text, ",")? {
                out.push(i32_to_exp_golomb(n));
            }
        } else {
            for n in string_to_u32s(text, ",")? {
                out.push(u32_to_exp_golomb(n));
            }
        }

        let sep = if self.spaced { ", " } else { "" };

        if self.invert {
            Ok(out.iter().map(|s| swap_01(s.to_string())).join(sep))
        } else {
            Ok(out.join(sep))
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

    const PTEXT: &'static str = "0, 1, 2, 3, 8, 1234, 4294967295";
    const PTEXT_SIGNED: &'static str = "0, -1, 1, -2, 4, 1234, -1234, -2147483647, 2147483647";
    const ENCODEDTEXT: &'static str = "101001100100000100100000000001001101001100000000000000000000000000000000100000000000000000000000000000000";
    const ENCODEDTEXT_SIGNED: &'static str = "10100110010000010010000000000010011010010100000000000100110100100000000000000000000000000000000011111111111111111111111111111110000000000000000000000000000000011111111111111111111111111111111";
    const ENCODEDTEXT_INV: &'static str = "010110011011111011011111111110110010110011111111111111111111111111111111011111111111111111111111111111111";
    const ENCODEDTEXT_SP: &'static str = "1, 010, 011, 00100, 0001001, 000000000010011010011, 00000000000000000000000000000000100000000000000000000000000000000";

    #[test]
    fn encode_test() {
        let mut code = ExpGolomb::default();
        assert_eq!(code.encode(PTEXT).unwrap(), ENCODEDTEXT);
        code.invert = true;
        assert_eq!(code.encode(PTEXT).unwrap(), ENCODEDTEXT_INV);
        code.invert = false;
        code.spaced = true;
        assert_eq!(code.encode(PTEXT).unwrap(), ENCODEDTEXT_SP);
    }

    #[test]
    fn encode_test_signed() {
        let mut code = ExpGolomb::default();
        code.signed = true;
        assert_eq!(code.encode(PTEXT_SIGNED).unwrap(), ENCODEDTEXT_SIGNED);
    }

    #[test]
    fn decode_test() {
        let mut code = ExpGolomb::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PTEXT);
        assert_eq!(code.decode(ENCODEDTEXT_SP).unwrap(), PTEXT);
        code.invert = true;
        assert_eq!(code.decode(ENCODEDTEXT_INV).unwrap(), PTEXT);
    }

    #[test]
    fn decode_test_signed() {
        let mut code = ExpGolomb::default();
        code.signed = true;
        assert_eq!(code.decode(ENCODEDTEXT_SIGNED).unwrap(), PTEXT_SIGNED);
    }
}
