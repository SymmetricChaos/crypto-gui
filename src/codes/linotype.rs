use super::Code;
use crate::{errors::Error, text_aux::text_functions::bimap_from_iter};
use bimap::BiMap;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::cell::Cell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MagazineVariant {
    NinetyChannel,
    SeventyTwoChannel,
}
//http://www.linotype.org/OnLineDocs/LinotypeMachinePrinciples-1940/LMP-chapter20.pdf
const LINOTYPE_90_MAG: &'static str =
    "taoinshrdlucmfwypvbgkqjxz\u{FB01}\u{FB02}\u{FB00}\u{FB03}\u{FB04}\u{2003},.:;?\u{2007}(|\"!-\u{2009})\u{2024}'*1234567890$\u{2025}ETAOINSHRDLUCMFWYPVBGKQJXZ@\u{00E6}&\u{2014}";

lazy_static! {
    pub static ref LINO_90_MAP: BiMap<char, String> = bimap_from_iter(
        LINOTYPE_90_MAG
            .chars()
            .zip((4..93).map(|n| format!("{:07b}", n)))
    );
}

pub struct Linotype {
    pub variant: MagazineVariant,
    first_e_channel: Cell<bool>,
}

impl Linotype {
    pub fn map_inv(&self, s: &str) -> Result<&char, Error> {
        if s == "0000010" {
            Ok(&'e')
        } else if s == "0000011" {
            Ok(&'e')
        } else {
            LINO_90_MAP
                .get_by_right(s)
                .ok_or(Error::invalid_input_group(s))
        }
    }

    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (char, String)> + '_> {
        let chars = "eetaoinshrdlucmfwypvbgkqjxz\u{FB01}\u{FB02}\u{FB00}\u{FB03}\u{FB04}␠,.:;?␠(|\"!-␠)\u{2024}'*1234567890$\u{2025}ETAOINSHRDLUCMFWYPVBGKQJXZ@\u{00E6}&\u{2014}".chars();
        Box::new((2..93).zip(chars).map(|(n, c)| (c, format!("{:07b}", n))))
    }
}

impl Default for Linotype {
    fn default() -> Self {
        Linotype {
            variant: MagazineVariant::NinetyChannel,
            first_e_channel: Cell::new(true),
        }
    }
}

impl Code for Linotype {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let mut out = String::new();
        for c in text.chars() {
            if c == 'e' {
                let channel = self.first_e_channel.get();
                self.first_e_channel.set(!channel);
                if channel {
                    out.push_str("0000010")
                } else {
                    out.push_str("0000011")
                }
            // For ease of use we'll map the ASCII space to the em-space
            } else if c == ' ' {
                out.push_str("0100010")
            } else {
                out.push_str(
                    LINO_90_MAP
                        .get_by_left(&c)
                        .ok_or(Error::invalid_input_char(c))?,
                )
            }
        }
        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut out = String::new();
        let binding = text.chars().filter(|c| !c.is_whitespace()).chunks(7);
        let chunks = binding.into_iter().map(|chunk| chunk.collect::<String>());
        for chunk in chunks {
            out.push(*self.map_inv(&chunk)?)
        }
        Ok(out)
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod linotype_tests {
    use super::*;

    // Multiple 'e' characters appear
    const PLAINTEXT: &'static str = "TheQuickBrownFoxJumpsOverTheLazyDog";
    const CIPHERTEXT: &'static str = "10000000001010000001010101010001110000011100011110011000101001000010110000110001001000010001001101000011000110111010110000111000100000010100000100110000100010101000001100010111000000000101000000101001001000010100111000010011100100000001100010111";

    #[test]
    fn encrypt_test() {
        let code = Linotype::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = Linotype::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT);
    }
}
