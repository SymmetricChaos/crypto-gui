use itertools::Itertools;
use num::CheckedAdd;

use super::{decode_prefix_to_strings, string_to_u32s, swap_01};
use crate::{errors::CodeError, traits::Code};

// First 46 Fibonacci numbers (skipping the initial 0 and 1), all the ones that fit in u32
pub const FIBS: [u32; 46] = [
    1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946,
    17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269, 2178309, 3524578,
    5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155, 165580141, 267914296,
    433494437, 701408733, 1134903170, 1836311903, 2971215073,
];

// https://en.wikipedia.org/wiki/Fibonacci_coding

pub struct FibonacciCode {
    pub spaced: bool,
    pub invert: bool,
    pub signed: bool,
}

impl Default for FibonacciCode {
    fn default() -> Self {
        FibonacciCode {
            spaced: false,
            invert: false,
            signed: false,
        }
    }
}

impl FibonacciCode {
    pub fn encode_u32(&self, n: u32) -> Option<String> {
        if n == 0 {
            return None;
        }

        let mut bits = String::from("1");

        let mut val = n;
        for f in FIBS.into_iter().filter(|x| x <= &n).rev() {
            if f <= val {
                bits.push('1');
                val -= f;
            } else {
                bits.push('0')
            }
        }

        // Reverse the bits, collect them into a String
        Some(bits.chars().rev().collect::<String>())
    }

    pub fn decode_to_u32(&self, text: &str) -> Vec<Option<u32>> {
        let (z0, z1) = if self.invert { ('1', '0') } else { ('0', '1') };
        let mut output = Vec::new();
        let mut prev = z0;
        let mut ctr = 0;
        let mut n = 0;
        let mut valid = true;
        for bit in text.chars() {
            if prev == z1 && bit == z1 {
                if valid {
                    output.push(Some(n));
                } else {
                    output.push(None);
                }
                prev = z0;
                ctr = 0;
                n = 0;
                valid = true;
                continue;
            }
            if bit == z0 {
                ()
            } else if bit == z1 {
                if let Some(f) = FIBS.get(ctr) {
                    if let Some(sum) = n.checked_add(f) {
                        n = sum;
                    } else {
                        valid = false
                    }
                } else {
                    valid = false
                };
            } else {
                valid = false
            }

            ctr += 1;
            prev = bit;
        }
        if n != 0 {
            output.push(Some(n));
        }
        output
    }

    fn decode_to_u32_single(&self, text: &str) -> Option<u32> {
        let o = self.decode_to_u32(text);
        if o.len() != 1 {
            return None;
        } else {
            return o[0];
        }
    }
}

impl Code for FibonacciCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::new();

        for n in string_to_u32s(text, ",")? {
            match self.encode_u32(n) {
                Some(code) => out.push(code.clone()),
                None => out.push(String::from("�")),
            }
        }

        let s = if self.spaced {
            out.join(", ")
        } else {
            out.join("")
        };

        if self.invert {
            Ok(swap_01(s))
        } else {
            Ok(s)
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::new();

        if self.spaced {
            for section in text.split(",").map(|s| s.trim()) {
                decode_prefix_to_strings(
                    self.decode_to_u32_single(&section),
                    self.signed,
                    &mut out,
                );
            }
        } else {
            for section in self.decode_to_u32(&text) {
                decode_prefix_to_strings(section, self.signed, &mut out);
            }
        }

        Ok(out.into_iter().join(", "))
    }
}

#[cfg(test)]
mod fibonacci_tests {
    use super::*;

    const PLAINTEXT: &'static str = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17";
    const ENCODEDTEXT: &'static str = "110110011101100011100110101100001110001101001100101110101100000111000011010001100100111010011";
    const ENCODEDTEXT_SP: &'static str = "11, 011, 0011, 1011, 00011, 10011, 01011, 000011, 100011, 010011, 001011, 101011, 0000011, 1000011, 0100011, 0010011, 1010011";
    const ENCODEDTEXT_INV: &'static str = "001001100010011100011001010011110001110010110011010001010011111000111100101110011011000101100";
    const ENCODEDTEXT_SP_INV: &'static str = "00, 100, 1100, 0100, 11100, 01100, 10100, 111100, 011100, 101100, 110100, 010100, 1111100, 0111100, 1011100, 1101100, 0101100";

    #[test]
    fn encode_tests() {
        let mut code = FibonacciCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
        code.spaced = true;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_SP);
        code.spaced = false;
        code.invert = true;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_INV);
        code.spaced = true;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_SP_INV);
    }

    #[test]
    fn decode_tests() {
        let mut code = FibonacciCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
        code.spaced = true;
        assert_eq!(code.decode(ENCODEDTEXT_SP).unwrap(), PLAINTEXT);
        code.spaced = false;
        code.invert = true;
        assert_eq!(code.decode(ENCODEDTEXT_INV).unwrap(), PLAINTEXT);
        code.spaced = true;
        assert_eq!(code.decode(ENCODEDTEXT_SP_INV).unwrap(), PLAINTEXT);
    }

    #[test]
    fn error_tests() {
        let code = FibonacciCode::default();
        assert_eq!("��", code.encode("0, 4000000000").unwrap());
        // for i in 1830000000..u32::MAX {
        //     if &code.encode(&format!("{i}")).unwrap() == "�" {
        //         println!("{i}");
        //         break;
        //     }
        // }
    }
}
