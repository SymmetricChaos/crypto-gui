use bimap::BiMap;
use itertools::Itertools;
use num::CheckedAdd;
use utils::preset_alphabet::Alphabet;

use crate::{errors::CodeError, traits::Code};

pub const FIBS: [u32; 46] = [
    1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946,
    17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269, 2178309, 3524578,
    5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155, 165580141, 267914296,
    433494437, 701408733, 1134903170, 1836311903, 2971215073,
];

const VARICODE: [&str; 128] = [
    "1010101011",
    "1011011011",
    "1011101101",
    "1101110111",
    "1011101011",
    "1101011111",
    "1011101111",
    "1011111101",
    "1011111111",
    "11101111",
    "11101",
    "1101101111",
    "1011011101",
    "11111",
    "1101110101",
    "1110101011",
    "1011110111",
    "1011110101",
    "1110101101",
    "1110101111",
    "1101011011",
    "1101101011",
    "1101101101",
    "1101010111",
    "1101111011",
    "1101111101",
    "1110110111",
    "1101010101",
    "1101011101",
    "1110111011",
    "1011111011",
    "1101111111",
    "1",
    "111111111",
    "101011111",
    "111110101",
    "111011011",
    "1011010101",
    "1010111011",
    "101111111",
    "11111011",
    "11110111",
    "101101111",
    "111011111",
    "1110101",
    "110101",
    "1010111",
    "110101111",
    "10110111",
    "10111101",
    "11101101",
    "11111111",
    "101110111",
    "101011011",
    "101101011",
    "110101101",
    "110101011",
    "110110111",
    "11110101",
    "110111101",
    "111101101",
    "1010101",
    "111010111",
    "1010101111",
    "1010111101",
    "1111101",
    "11101011",
    "10101101",
    "10110101",
    "1110111",
    "11011011",
    "11111101",
    "101010101",
    "1111111",
    "111111101",
    "101111101",
    "11010111",
    "10111011",
    "11011101",
    "10101011",
    "11010101",
    "111011101",
    "10101111",
    "1101111",
    "1101101",
    "101010111",
    "110110101",
    "101011101",
    "101110101",
    "101111011",
    "1010101101",
    "111110111",
    "111101111",
    "111111011",
    "1010111111",
    "101101101",
    "1011011111",
    "1011",
    "1011111",
    "101111",
    "101101",
    "11",
    "111101",
    "1011011",
    "101011",
    "1101",
    "111101011",
    "10111111",
    "11011",
    "111011",
    "1111",
    "111",
    "111111",
    "110111111",
    "10101",
    "10111",
    "101",
    "110111",
    "1111011",
    "1101011",
    "11011111",
    "1011101",
    "111010101",
    "1010110111",
    "110111011",
    "1010110101",
    "1011010111",
    "1110110101",
];

crate::lazy_bimap!(
    VARICODE_MAP: BiMap<char, &str> = Alphabet::Ascii128.chars().zip(VARICODE);
);

pub struct Varicode {
    pub spaced: bool,
}

impl Default for Varicode {
    fn default() -> Self {
        Varicode { spaced: false }
    }
}

impl Varicode {
    pub fn map(&self, c: char) -> Result<String, CodeError> {
        if c.is_ascii_control() {
            return Ok(VARICODE[c as usize].to_string());
        }
        match VARICODE_MAP.get_by_left(&c) {
            Some(s) => Ok(s.to_string()),
            None => Err(CodeError::invalid_alphabet_char(c)),
        }
    }

    pub fn map_inv(&self, s: &str) -> Result<char, CodeError> {
        match VARICODE_MAP.get_by_right(&s) {
            Some(c) => Ok(*c),
            None => Err(CodeError::invalid_input_group(s)),
        }
    }

    pub fn parse_codes(&self, text: &str) -> Vec<Option<String>> {
        let (z0, z1) = ('1', '0');
        let mut output = Vec::new();
        let mut prev = z0;
        let mut ctr = 0;
        let mut head = 0;
        let mut n = 0;
        let mut valid = true;
        for bit in text.chars() {
            if prev == z1 && bit == z1 {
                if valid {
                    // println!("{}", &text[head..(head + ctr - 1)]);
                    output.push(Some(text[head..(head + ctr - 1)].to_string()));
                } else {
                    // println!("INVALID GROUP");
                    output.push(None);
                }
                prev = z0;
                head = head + ctr + 1;
                // println!("moved head to {head}");
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
            // println!("{}", &text[head..]);
            output.push(Some(text[head..].to_string()));
        }
        output
    }

    pub fn chars_codes_display(&self) -> Box<dyn Iterator<Item = (char, &&str)> + '_> {
        Box::new(
            Alphabet::Ascii128
                .chars()
                .map(|c| (c, VARICODE_MAP.get_by_left(&c).unwrap())),
        )
    }
}

impl Code for Varicode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::new();
        for c in text.chars() {
            out.push(self.map(c)?)
        }
        if self.spaced {
            Ok(out.into_iter().join(" "))
        } else {
            Ok(out.into_iter().join("00"))
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let text = text.replace(" ", "");
        let codes = self.parse_codes(&text);
        let mut out = String::new();
        for s in codes {
            match s {
                // The control pictures have the last same eight bits as the actual control characters.
                // Casting to u8 is guaranteed keep only the last eight bits so it automatically maps control pictures to control characters.
                Some(code) => out.push((self.map_inv(&code)? as u8) as char),
                None => return Err(CodeError::input("impossible input group")),
            }
        }
        Ok(out)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_short() {
        let mut code = Varicode::default();
        code.spaced = true;
        let ctext = code.encode("ABCD").unwrap();
        assert_eq!("ABCD", code.decode(&ctext).unwrap());
    }
}
