use std::str::Chars;

use itertools::Itertools;
use lazy_static::lazy_static;

use crate::{binary_to_text::BinaryToText, errors::CodeError, traits::Code};
use utils::{
    byte_formatting::{bytes_to_hex_be, ByteFormat},
    text_functions::string_chunks,
};

// \u{00A0} is nonbreaking space. \u{00AD} is soft hyphen.
pub const CP1252: &'static str = "␀␁␂␃␄␅␆␇␈␉␊␋␌␍␎␏␐␑␒␓␔␕␖␗␘␙␚␛␜␝␞␟ !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~␡€�‚ƒ„…†‡ˆ‰Š‹Œ�Ž��‘’“”•–—˜™š›œ�žŸ\u{00A0}¡¢£¤¥¦§¨©ª«¬\u{00AD}®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ";
pub const CP437: &'static str = "␀☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼ !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■\u{00A0}";

lazy_static! {
    // pub static ref BYTES: Vec<u8> = (0..=255u8).collect_vec();
    pub static ref BINARY: Vec<String> = (0..256).map(|n| format!("{:08b}", n)).collect_vec();
    pub static ref OCTAL: Vec<String> = (0..256).map(|n| format!("{:03o}", n)).collect_vec();
    pub static ref DECIMAL: Vec<String> = (0..256).map(|n| format!("{:03}", n)).collect_vec();
    pub static ref HEX: Vec<String> = (0..256).map(|n| format!("{:02x}", n)).collect_vec();
}

#[derive(Debug, PartialEq, Eq)]
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
    pub fn name(&self) -> &str {
        match self {
            DisplayMode::Binary => "Binary",
            DisplayMode::Octal => "Octal",
            DisplayMode::Decimal => "Decimal",
            DisplayMode::Hex => "Hexadecimal",
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
    pub b2t_mode: Option<ByteFormat>,
}

impl BinaryToText for Ccsid {
    fn encode_bytes(&self, bytes: &[u8]) -> Result<String, CodeError> {
        Ok(bytes
            .into_iter()
            .map(|b| self.page.chars().nth(*b as usize).unwrap())
            .collect())
    }
}

impl Ccsid {
    pub fn map(&self, c: char) -> Result<&String, CodeError> {
        if c == '�' {
            return Err(CodeError::invalid_input_char(c));
        };
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

    pub fn decode_to_bytes(&self, text: &str) -> Result<String, CodeError> {
        let out = text
            .chars()
            .map(|c| self.page.chars().position(|x| x == c).unwrap() as u8)
            .collect_vec();
        match self.b2t_mode {
            Some(ByteFormat::Hex) => Ok(bytes_to_hex_be(&out)),
            Some(ByteFormat::Utf8) => {
                String::from_utf8(out).map_err(|e| CodeError::Input(e.to_string()))
            }
            Some(ByteFormat::Base64) => todo!(),
            Some(ByteFormat::Bit) => todo!(),
            None => Err(CodeError::state("Binary to Text Mode is not set")),
            _ => todo!(),
        }
    }
}

impl Default for Ccsid {
    fn default() -> Self {
        Self {
            mode: DisplayMode::Binary,
            page: CodePage::CP1252,
            spaced: false,
            b2t_mode: None,
        }
    }
}

impl Code for Ccsid {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        if let Some(m) = self.b2t_mode {
            match m {
                ByteFormat::Hex => self.encode_hex(text),
                ByteFormat::Utf8 => self.encode_utf8(text),
                _ => todo!(),
            }
        } else {
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
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        if self.b2t_mode.is_some() {
            self.decode_to_bytes(text)
        } else {
            let chunks = string_chunks(&text.replace(' ', ""), self.mode.width());
            let mut out = String::with_capacity(chunks.len());

            for chunk in chunks {
                out.push(self.map_inv(&chunk)?)
            }

            Ok(out)
        }
    }
}

#[cfg(test)]
mod ccsid_tests {
    use super::*;

    #[test]
    #[ignore = "pairing"]
    fn test_pairing() {
        println!("CP1252");
        let mut code = Ccsid::default();
        for line in code.chars_codes() {
            println!("{}  {}", line.0, line.1)
        }
        println!("\n\nCP437");
        code.page = CodePage::CP437;
        for line in code.chars_codes() {
            println!("{}  {}", line.0, line.1)
        }
    }
}
