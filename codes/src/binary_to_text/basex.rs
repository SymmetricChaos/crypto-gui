use super::{bytes_to_hex, BinaryToText, BinaryToTextMode};
use crate::{errors::CodeError, traits::Code};
use bimap::BiMap;
use num::Zero;
use utils::text_functions::bimap_from_iter;

// Make it possible to encode an aribtrary file
pub struct BaseX {
    pub mode: BinaryToTextMode,
    pub base: u32,
    map: BiMap<char, u32>,
}

impl Default for BaseX {
    fn default() -> Self {
        let map = bimap_from_iter(
            "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
                .chars()
                .enumerate()
                .map(|(n, c)| (c, n as u32)),
        );
        Self {
            mode: BinaryToTextMode::Utf8,
            base: 62,
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

        let leading_zeroes = digits.iter().take_while(|n| n.is_zero()).count();

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
        let mut out: Vec<u8> = Vec::new();
        let mut chars = text.chars().filter(|c| !c.is_whitespace()).peekable();

        loop {
            // Break if done
            if chars.peek().is_none() {
                break;
            }

            // Handle special 'z' and 'y' characters
            if *chars.peek().unwrap() == 'z' {
                out.extend_from_slice(&[0, 0, 0, 0]);
                chars.next(); // remove the 'z'
                continue;
            }
            if *chars.peek().unwrap() == 'y' {
                out.extend_from_slice(&[0x20, 0x20, 0x20, 0x20]);
                chars.next(); // remove the 'y'
                continue;
            }

            // If those are handled we fill the buffer algebraically
            let mut buffer = 0_u32;

            let mut used_chars = 4;
            for i in (0..5).rev() {
                match chars.next() {
                    Some(byte) => {
                        buffer += *self
                            .map
                            .get_by_right(&(byte as u32))
                            .ok_or_else(|| CodeError::invalid_input_char(byte as char))?
                            as u32
                            * 85_u32.pow(i)
                    }
                    None => {
                        used_chars -= 1;
                        buffer += 84 * 85_u32.pow(i);
                    }
                }
            }

            // Extract the used bytes from the buffer
            for b in buffer.to_le_bytes().into_iter().rev().take(used_chars) {
                out.push(b)
            }
        }
        match self.mode {
            BinaryToTextMode::Hex => bytes_to_hex(&out),
            BinaryToTextMode::Utf8 => {
                String::from_utf8(out).map_err(|e| CodeError::Input(e.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod ascii85_tests {
    use super::*;

    const TESTS: [(&'static str, &'static str); 8] = [
        ("Man is d", "9jqo^BlbD-"),      // multiple blocks
        ("Man ", "9jqo^"),               // single block
        ("Man", "9jqo"),                 // partial
        ("Ma", "9jn"),                   // partial
        ("M", "9`"),                     // partial
        ("    ", "y"),                   // special
        ("\0\0\0\0", "z"),               // special
        ("abcd    efgh", "@:E_WyAS,Rg"), // special in contex
    ];

    #[test]
    fn encode_test() {
        let code = BaseX::default();
        for (ptext, ctext) in TESTS {
            assert_eq!(code.encode(ptext).unwrap(), ctext);
        }
    }

    #[test]
    fn decode_test() {
        let code = BaseX::default();
        for (ptext, ctext) in TESTS {
            assert_eq!(code.decode(ctext).unwrap(), ptext);
        }
    }

    #[test]
    fn decode_test_errs() {
        let code = BaseX::default();
        // Fail on character that is always invalid
        assert_eq!(
            code.decode("abdc}").unwrap_err(),
            CodeError::Input("invalid character `}`, alphabets are case sensitive".into())
        );
        // Fail on z if not found at the start of a chunk
        assert_eq!(
            code.decode("azg}").unwrap_err(),
            CodeError::Input("invalid character `z`, alphabets are case sensitive".into())
        );
        // Fail on y if not found at the start of a chunk
        assert_eq!(
            code.decode("agy{").unwrap_err(),
            CodeError::Input("invalid character `y`, alphabets are case sensitive".into())
        );
    }
}
