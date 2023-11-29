use std::str::Chars;

use crate::{errors::CodeError, traits::Code};
use bimap::BiMap;
use itertools::Itertools;
use lazy_static::lazy_static;
use utils::text_functions::{bimap_from_iter, string_chunks};

// Additional space is the non-breaking space. Additional hyphen is the soft hypen.
pub const CP1252: &'static str = "␀␁␂␃␄␅␆␇␈␉␊␋␌␍␎␏␐␑␒␓␔␕␖␗␘␙␚␛␜␝␞␟ !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~␡€�‚ƒ„…†‡ˆ‰Š‹Œ�Ž��‘’“”•–—˜™š›œ�žŸ ¡¢£¤¥¦§¨©ª«¬-®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ";
// Additional space is the non-breaking space.
pub const CP437: &'static str = "␀☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼ !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■ ";

lazy_static! {
    pub static ref BINARY: Vec<String> = (0..256).map(|n| format!("{:08b}", n)).collect_vec();
    pub static ref OCTAL: Vec<String> = (0..256).map(|n| format!("{:03o}", n)).collect_vec();
    pub static ref DECIMAL: Vec<String> = (0..256).map(|n| format!("{:03}", n)).collect_vec();
    pub static ref HEX: Vec<String> = (0..256).map(|n| format!("{:02x}", n)).collect_vec();
}

pub enum CodePage {
    CP1252,
    CP437,
}

impl CodePage {
    pub fn chars(&self) -> Chars<'_> {
        match self {
            CodePage::CP1252 => CP1252.chars(),
            CodePage::CP437 => CP437.chars(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    Binary,
    Octal,
    Decimal,
    Hex,
}

impl DisplayMode {
    pub fn radix(&self) -> u32 {
        match self {
            DisplayMode::Binary => 2,
            DisplayMode::Octal => 8,
            DisplayMode::Decimal => 10,
            DisplayMode::Hex => 16,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            DisplayMode::Binary => "binary",
            DisplayMode::Octal => "octal",
            DisplayMode::Decimal => "decimal",
            DisplayMode::Hex => "hexadecimal",
        }
    }

    pub fn width(&self) -> usize {
        match self {
            DisplayMode::Binary => 8,
            DisplayMode::Octal => 3,
            DisplayMode::Decimal => 3,
            DisplayMode::Hex => 2,
        }
    }
}

pub struct Ccsid {
    pub mode: DisplayMode,
    pub page: CodePage,
    pub spaced: bool,
}

impl Ccsid {
    pub fn map(&self, c: char) -> Result<&String, CodeError> {
        let n = self
            .page
            .chars()
            .position(|x| x == c)
            .ok_or(CodeError::invalid_input_char(c))?;
        match self.mode {
            DisplayMode::Binary => Ok(BINARY.get(n).unwrap()),
            DisplayMode::Octal => Ok(OCTAL.get(n).unwrap()),
            DisplayMode::Decimal => Ok(DECIMAL.get(n).unwrap()),
            DisplayMode::Hex => Ok(HEX.get(n).unwrap()),
        }
    }

    pub fn map_inv(&self, s: &str) -> Result<char, CodeError> {
        let n = match self.mode {
            DisplayMode::Binary => BINARY.iter().position(|x| x == s),
            DisplayMode::Octal => OCTAL.iter().position(|x| x == s),
            DisplayMode::Decimal => DECIMAL.iter().position(|x| x == s),
            DisplayMode::Hex => HEX.iter().position(|x| x == s),
        }
        .ok_or(CodeError::invalid_input_group(s))?;

        Ok(self.page.chars().nth(n).unwrap())
    }

    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (char, &String)> + '_> {
        let cs = self.page.chars();
        match self.mode {
            DisplayMode::Binary => Box::new(cs.zip(BINARY.iter())),
            DisplayMode::Octal => Box::new(cs.zip(OCTAL.iter())),
            DisplayMode::Decimal => Box::new(cs.zip(DECIMAL.iter())),
            DisplayMode::Hex => Box::new(cs.zip(HEX.iter())),
        }
    }
}

impl Default for Ccsid {
    fn default() -> Self {
        Self {
            mode: DisplayMode::Binary,
            page: CodePage::CP1252,
            spaced: false,
        }
    }
}

impl Code for Ccsid {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::new();
        for c in text.chars() {
            out.push(self.map(c)?)
        }
        if self.spaced {
            Ok(out.into_iter().join(" "))
        } else {
            Ok(out.into_iter().join(""))
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let chunks = string_chunks(&text.replace(' ', ""), self.mode.width());
        let mut out = String::with_capacity(chunks.len());

        for chunk in chunks {
            out.push(self.map_inv(&chunk)?)
        }

        Ok(out)
    }
}

#[cfg(test)]
mod ccsid_tests {
    use super::*;

    const PLAINTEXT: &'static str = "þ";
    const CIPHERTEXT: &'static str = "11111110";

    #[test]
    fn encode_test() {
        let code = Ccsid::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn encrypt_decrypt_test() {
        let mut code = Ccsid::default();
        const GIVEN_TEXT: &'static str = "The quick␠brown fox!␀␀␀Jumps over the lazy(dog)";
        const DECODED_TEXT: &'static str = "The quick brown fox!␀␀␀Jumps over the lazy(dog)";

        for mode in [
            DisplayMode::Binary,
            DisplayMode::Octal,
            DisplayMode::Decimal,
            DisplayMode::Hex,
        ] {
            code.mode = mode;
            let encoded = code
                .encode(GIVEN_TEXT)
                .expect(&format!("encoding ASCII {:?} error", mode));
            let decoded = code
                .decode(&encoded)
                .expect(&format!("decoding ASCII {:?} error", mode));
            if decoded != DECODED_TEXT {
                panic!(
                    "decoded ASCII {:?} not equivalent to plaintext\n{}",
                    mode, decoded
                )
            }
        }
    }
}
