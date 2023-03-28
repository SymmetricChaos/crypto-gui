use crate::{
    errors::Error,
    text_aux::{bytes_as_text::NumRep, PresetAlphabet::ClassicalLatin, VecString},
};

use super::{BlockCode, Code};

pub struct Bacon {
    pub block: BlockCode,
    pub false_text: String,
}

impl Default for Bacon {
    fn default() -> Self {
        let mut block = BlockCode::default();
        block.rep = NumRep::Binary;
        block.width = 5;
        block.symbols = VecString::from(ClassicalLatin);
        Bacon {
            block,
            false_text: String::new(),
        }
    }
}

impl Bacon {
    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (char, String)> + '_> {
        self.block.chars_codes()
    }
}

impl Code for Bacon {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let usable_chars = self
            .false_text
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .count();
        let chars_needed = text.chars().count() * self.block.width;
        if usable_chars < chars_needed {
            return Err(Error::Input(format!(
                "At least {chars_needed} alphabetic characters are needed in the false text."
            )));
        }
        let binding = self.block.encode(text)?;
        let mut bits = binding.chars().map(|c| if c == '0' { false } else { true });
        let mut out = String::new();
        for c in self.false_text.chars() {
            if c.is_ascii_alphabetic() {
                if let Some(b) = bits.next() {
                    if b {
                        out.push(c.to_ascii_uppercase());
                    } else {
                        out.push(c.to_ascii_lowercase());
                    }
                }
            }
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        self.block.decode(text)
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod bacon_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQVICKBROWNFOXIUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "1001100111001001000010100010000001001010000011000101110101100110100101011101011101001101000110001111100100111010101001001000110011001110010001011000001100111000000110111000110";

    #[test]
    fn encrypt_test() {
        let code = Bacon::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = Bacon::default();
        assert_eq!(code.decode(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
