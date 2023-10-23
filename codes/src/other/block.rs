use itertools::Itertools;
use num::{Integer, Zero};
use utils::preset_alphabet::Alphabet;

use crate::{errors::CodeError, traits::Code};

fn to_str_radix(n: usize, radix: usize, width: usize, symbols: &Vec<char>) -> String {
    if n.is_zero() {
        return String::from(symbols[0]).repeat(width);
    }

    let mut values = Vec::with_capacity(width);

    let mut n = n;
    while n != 0 || values.len() < width {
        let (q, r) = n.div_mod_floor(&radix);
        values.push(r);
        n = q;
    }

    values.into_iter().map(|x| symbols[x]).rev().collect()
}

pub struct BlockCode {
    pub width: usize,
    pub alphabet: Vec<char>,
    pub symbols: Vec<char>,
    pub spaced: bool,
}

impl Default for BlockCode {
    fn default() -> Self {
        BlockCode {
            width: 5,
            alphabet: Alphabet::BasicLatin.chars().collect_vec(),
            symbols: vec!['0', '1'],
            spaced: false,
        }
    }
}

impl BlockCode {
    fn num_to_string(&self, n: usize) -> String {
        to_str_radix(n, self.symbols.len(), self.width, &self.symbols)
    }

    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (&char, String)> + '_> {
        Box::new(
            self.alphabet
                .iter()
                .enumerate()
                .map(|(n, c)| (c, self.num_to_string(n)))
                .take(self.total_codes()),
        )
    }

    pub fn total_codes(&self) -> usize {
        self.symbols.len().pow(self.width as u32)
    }

    pub fn min_code_width(&self) -> usize {
        (self.alphabet.len() as f32)
            .log(self.symbols.len() as f32)
            .ceil() as usize
    }

    pub fn check_code_width(&self) -> Result<(), CodeError> {
        let min_width = self.min_code_width();
        if min_width < self.symbols.len() {
            Err(CodeError::State(format!(
                "a width of {} is needed to represent the entire alphabet",
                min_width
            )))
        } else {
            Ok(())
        }
    }
}

impl Code for BlockCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::with_capacity(text.len());
        for c in text.chars() {
            let n = self
                .alphabet
                .iter()
                .position(|x| x == &c)
                .ok_or_else(|| CodeError::invalid_input_char(c))?;
            if n > self.total_codes() {
                return Err(CodeError::invalid_input_char(c));
            }
            out.push(self.num_to_string(n));
        }
        if self.spaced {
            Ok(out.join(" "))
        } else {
            Ok(out.join(""))
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();
        let text = text.replace(" ", "");
        let pows = (0..self.width).rev().cycle();
        let mut val = 0;
        for (c, p) in text.chars().zip(pows) {
            let n = self
                .symbols
                .iter()
                .position(|x| x == &c)
                .ok_or(CodeError::invalid_input_char(c))?;
            val += n * self.symbols.len().pow(p as u32);

            if p == 0 {
                if val > self.total_codes() {
                    return Err(CodeError::input("unable to decode"));
                }
                out.push(
                    *self
                        .alphabet
                        .iter()
                        .nth(val)
                        .ok_or(CodeError::input("unable to decode"))?,
                );
                val = 0;
            }
        }

        Ok(out)
    }
}

#[cfg(test)]
mod block_code_tests {
    use super::*;

    const PLAINTEXT: &'static str = "ABC";
    const CIPHERTEXT_01: &'static str = "000000000100010";
    const CIPHERTEXT_XYZ: &'static str = "XXXYXZ";

    #[test]
    fn encode_test_default() {
        let code = BlockCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT_01);
    }

    #[test]
    fn decode_test_default() {
        let code = BlockCode::default();
        assert_eq!(code.decode(CIPHERTEXT_01).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encode_test() {
        let mut code = BlockCode::default();
        code.symbols = "XYZ".chars().collect();
        code.width = 2;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT_XYZ);
    }

    #[test]
    fn decode_test() {
        let mut code = BlockCode::default();
        code.symbols = "XYZ".chars().collect();
        code.width = 2;
        assert_eq!(code.decode(CIPHERTEXT_XYZ).unwrap(), PLAINTEXT);
    }
}
