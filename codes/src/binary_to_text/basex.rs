use super::{BinaryToText, BinaryToTextMode};
use crate::{errors::CodeError, traits::Code};
use bimap::BiMap;
use itertools::Itertools;
use num::Zero;
use utils::text_functions::{bimap_from_iter, bytes_to_hex};

// Translated from
// https://github.com/eknkc/basex/blob/6baac8ea8b19cc66d125286d213770fec0691867/basex.go#L46

pub struct BaseX {
    pub mode: BinaryToTextMode,
    base: u32,
    map: BiMap<char, u32>,
}

impl Default for BaseX {
    fn default() -> Self {
        let map = bimap_from_iter(
            "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
                .chars()
                .enumerate()
                .map(|(n, c)| (c, n as u32)),
        );
        Self {
            mode: BinaryToTextMode::Utf8,
            base: 58,
            map,
        }
    }
}

impl BaseX {
    pub fn chars_codes(&self) -> impl Iterator<Item = (String, char)> + '_ {
        (0..self.map.len()).map(|x| {
            (
                format!("{x: <2}"),
                *self.map.get_by_right(&(x as u32)).unwrap(),
            )
        })
    }

    pub fn set_map(&mut self, alphabet: &str) {
        self.map = bimap_from_iter(alphabet.chars().enumerate().map(|(n, c)| (c, n as u32)));
        self.base = self.map.len() as u32;
    }

    pub fn base(&self) -> u32 {
        self.base
    }
}

impl BinaryToText for BaseX {
    fn encode_bytes(&self, bytes: &[u8]) -> Result<String, CodeError> {
        if bytes.len() == 0 {
            return Ok(String::new());
        }

        let mut digits = Vec::new();

        for b in bytes {
            let mut carry = *b as u32;
            for i in 0..digits.len() {
                carry += digits[i] << 8;
                digits[i] = carry % self.base;
                carry = carry / self.base;
            }
            while carry > 0 {
                digits.push(carry % self.base);
                carry = carry / self.base;
            }
        }

        let leading_zeroes = bytes.iter().take_while(|n| n.is_zero()).count();

        let mut out = format!("{}", self.map.get_by_right(&0).unwrap()).repeat(leading_zeroes);
        out.push_str(
            &digits
                .iter()
                .rev()
                .map(|n| self.map.get_by_right(n).unwrap())
                .collect::<String>(),
        );

        Ok(out)
    }
}

impl Code for BaseX {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        match self.mode {
            BinaryToTextMode::Hex => self.encode_hex(text),
            BinaryToTextMode::Utf8 => self.encode_utf8(text),
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut bytes: Vec<u8> = Vec::new();
        for c in text.chars() {
            let mut carry = *self
                .map
                .get_by_left(&c)
                .ok_or_else(|| CodeError::invalid_input_char(c))?;
            for j in 0..bytes.len() {
                carry += bytes[j] as u32 * self.base;
                bytes[j] = (carry & 0xFF) as u8;
                carry >>= 8;
            }

            while carry > 0 {
                bytes.push((carry & 0xFF) as u8);
                carry >>= 8;
            }
        }

        for _ in 0..text
            .chars()
            .take_while(|n| n == self.map.get_by_right(&0).unwrap())
            .count()
        {
            bytes.push(0);
        }

        let bytes = bytes.into_iter().rev().collect_vec();

        match self.mode {
            BinaryToTextMode::Hex => Ok(bytes_to_hex(&bytes)),
            BinaryToTextMode::Utf8 => {
                String::from_utf8(bytes).map_err(|e| CodeError::Input(e.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod basex_tests {
    use super::*;

    const TESTS: &[(&'static str, &'static str)] =
        &[("Man is d", "DwgwXHnykZ9"), ("Man ", "2yimnw"), ("a", "2g")];

    const HEX_TESTS: &[(&'static str, &'static str)] = &[("0000287fb4cd", "11233QC4")];

    #[test]
    fn encode_test() {
        let mut code = BaseX::default();
        for (ptext, ctext) in TESTS {
            assert_eq!(code.encode(ptext).unwrap(), *ctext);
        }
        code.mode = BinaryToTextMode::Hex;
        for (ptext, ctext) in HEX_TESTS {
            assert_eq!(code.encode(ptext).unwrap(), *ctext);
        }
    }

    #[test]
    fn decode_test() {
        let mut code = BaseX::default();
        for (ptext, ctext) in TESTS {
            assert_eq!(code.decode(ctext).unwrap(), *ptext);
        }
        code.mode = BinaryToTextMode::Hex;
        for (ptext, ctext) in HEX_TESTS {
            assert_eq!(code.decode(ctext).unwrap(), *ptext);
        }
    }
}
