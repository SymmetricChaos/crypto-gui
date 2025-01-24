use crate::{errors::CodeError, traits::Code};
use itertools::Itertools;
use std::cell::RefCell;

use super::{fibonacci_integers::FibonacciCodeIntegers, string_to_u32s, swap_01};

// https://en.wikipedia.org/wiki/Fibonacci_coding

pub struct FibonacciCode {
    pub integer_code: RefCell<FibonacciCodeIntegers>,
    pub spaced: bool,
    pub invert: bool,
}

impl Default for FibonacciCode {
    fn default() -> Self {
        let codes = FibonacciCodeIntegers::default();
        FibonacciCode {
            integer_code: RefCell::new(codes),
            spaced: false,
            invert: false,
        }
    }
}

impl Code for FibonacciCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut output = Vec::new();

        for n in string_to_u32s(text, ",")? {
            output.push(self.integer_code.borrow_mut().encode_u32(n).clone());
        }

        let s = if self.spaced {
            output.join(", ")
        } else {
            output.join("")
        };

        if self.invert {
            Ok(swap_01(s))
        } else {
            Ok(s)
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let text: String = if self.invert {
            swap_01(
                text.chars()
                    .filter(|c| *c == '0' || *c == '1')
                    .collect::<String>(),
            )
        } else {
            text.chars().filter(|c| *c == '0' || *c == '1').collect()
        };

        let nums = self.integer_code.borrow_mut().decode_to_u32(&text)?;

        Ok(nums.into_iter().map(|n| n.to_string()).join(", "))
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
}
