use itertools::Itertools;
use utils::preset_alphabet::PresetAlphabet;

use crate::{block::BlockCode, errors::CodeError, traits::Code};

pub struct Bacon {
    pub block: BlockCode,
    pub false_text: String,
}

impl Default for Bacon {
    fn default() -> Self {
        let mut block = BlockCode::default();
        block.width = 5;
        block.alphabet = PresetAlphabet::ClassicalLatin.chars().collect_vec();
        block.symbols = vec!['0', '1'];
        Bacon {
            block,
            false_text: String::new(),
        }
    }
}

impl Bacon {
    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (&char, String)> + '_> {
        self.block.chars_codes()
    }

    fn enough_false_text(&self, text: &str) -> Result<(), CodeError> {
        let usable_chars = self
            .false_text
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .count();
        let chars_needed = text.chars().count() * self.block.width;
        if usable_chars < chars_needed {
            return Err(CodeError::Input(format!(
                "At least {chars_needed} ASCII alphabetic characters are needed in the false text."
            )));
        }
        Ok(())
    }
}

fn capitalization_to_bits(c: char) -> char {
    if c.is_ascii_uppercase() {
        '1'
    } else {
        '0'
    }
}

fn bits_to_capitalization(c: char) -> bool {
    if c == '1' {
        true
    } else {
        false
    }
}

impl Code for Bacon {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        self.enough_false_text(text)?;

        let binding = self.block.encode(text)?;
        let mut bits = binding
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .map(|c| bits_to_capitalization(c));
        let mut out = String::new();

        for c in self.false_text.chars() {
            if c.is_ascii_alphabetic() {
                if let Some(b) = bits.next() {
                    if b {
                        out.push(c.to_ascii_uppercase());
                    } else {
                        out.push(c.to_ascii_lowercase());
                    }
                } else {
                    out.push(c.to_ascii_lowercase())
                }
            } else {
                out.push(c)
            }
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let usable_letters = text.chars().filter(|c| c.is_ascii_alphabetic()).count();
        let extra_letters = usable_letters % self.block.width;
        let message_length = usable_letters - extra_letters;
        let bits = text
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .take(message_length)
            .map(|c| capitalization_to_bits(c))
            .chunks(self.block.width)
            .into_iter()
            .map(|ch| ch.collect::<String>())
            .join("");
        self.block.decode(&bits)
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod bacon_tests {

    use super::*;

    const PLAINTEXT: &'static str = "ATTACKAAA";
    const FALSETEXT: &'static str = "There is nothing at all suspicious about this message.";
    const CIPHERTEXT: &'static str = "there Is nOtHinG at all suspIciOus About this message.";

    #[test]
    fn encrypt_test() {
        let mut code = Bacon::default();
        code.false_text = FALSETEXT.into();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = Bacon::default();
        assert_eq!(code.decode(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
