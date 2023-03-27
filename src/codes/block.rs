use crate::{
    errors::CodeError,
    text_aux::{
        bytes_as_text::{num_to_string_width, u32_from_string, ByteRep},
        PresetAlphabet::{self, BasicLatin},
        VecString,
    },
};
use lazy_static::lazy_static;
use std::collections::HashMap;

use super::Code;

pub struct BlockCode {
    width: u8,
    rep: ByteRep,
    symbols: VecString, // enforce comma seperated values
}

impl Default for BlockCode {
    fn default() -> Self {
        BlockCode {
            width: 5,
            rep: ByteRep::Binary,
            symbols: VecString::from(PresetAlphabet::BasicLatin),
        }
    }
}

impl BlockCode {
    pub fn assign_width(&mut self, width: u8) {
        if width >= 3 && width <= 8 {
            self.width = width
        }
    }

    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (char, &String)> + '_> {
        Box::new(
            self.symbols
                .chars()
                .map(|c| (c, num_to_string_width(&u32::from(c), self.rep, self.width))),
        )
    }

    pub fn check_code_width(&self) {
        let n_symbols = self.symbols.chars().count();
        let min_width = (n_symbols as f32).log(self.rep.radix()).ceil();
    }
}

impl Code for BlockCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::with_capacity(text.len());
        for c in text.chars() {
            let n = self.symbols.get_pos(c).ok_or_else(|| {
                Err(CodeError::Input(format!(
                    "The symbol `{c}` is not in the alphabet selected",
                )))
            })?;
            out.push(num_to_string_width(&n, self.rep, self.width));
        }
        Ok(out.join(" "))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();

        for group in text.split(" ") {
            let n = u32_from_string(group, self.rep).map_err(|e| {
                Err(CodeError::Input(format!(
                    "The code group `{group}` is not valid"
                )))
            })? as usize;
            out.push(
                self.symbols
                    .get_char_at(n)
                    .expect("tried to access byte outside alphabet range"),
            )
        }

        Ok(out)
    }
}

#[cfg(test)]
mod block_code_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "";

    #[test]
    fn encrypt_test() {
        let code = BlockCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = BlockCode::default();
        assert_eq!(code.decode(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
