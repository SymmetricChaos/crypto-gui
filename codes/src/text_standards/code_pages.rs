use crate::{errors::CodeError, traits::Code};
use bimap::BiMap;
use itertools::Itertools;
use lazy_static::lazy_static;
use utils::{
    preset_alphabet::Alphabet,
    text_functions::{bimap_from_iter, string_chunks},
};

pub const CP1252: &'static str = "␀␁␂␃␄␅␆␇␈␉␊␋␌␍␎␏␐␑␒␓␔␕␖␗␘␙␚␛␜␝␞␟ !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~␡€�‚ƒ„…†‡ˆ‰Š‹Œ�Ž��‘’“”•–—˜™š›œžŸ ¡¢£¤¥¦§¨©ª«¬SHY®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ";
pub const CP437: &'static str = "␀☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼ !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■ ";

// lazy_static! {
//     pub static ref
// }

pub enum CodePage {
    CP1252,
    CP437,
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
        if c.is_ascii() {
            return match self.mode {
                DisplayMode::EightBitBinary => EIGHT_BIT.get(c as u8 as usize),
                DisplayMode::SevenBitBinary => SEVEN_BIT.get(c as u8 as usize),
                DisplayMode::Octal => OCTAL.get(c as u8 as usize),
                DisplayMode::Decimal => DECIMAL.get(c as u8 as usize),
                DisplayMode::Hex => HEX.get(c as u8 as usize),
            }
            .ok_or(CodeError::invalid_input_char(c));
        }
        if let Some(control_val) = CONTROL_PICTURE_MAP.get_by_right(&c) {
            match self.mode {
                DisplayMode::EightBitBinary => EIGHT_BIT.get(*control_val as usize),
                DisplayMode::SevenBitBinary => SEVEN_BIT.get(*control_val as usize),
                DisplayMode::Octal => OCTAL.get(*control_val as usize),
                DisplayMode::Decimal => DECIMAL.get(*control_val as usize),
                DisplayMode::Hex => HEX.get(*control_val as usize),
            }
            .ok_or(CodeError::invalid_input_char(c))
        } else {
            Err(CodeError::invalid_input_char(c))
        }
    }

    pub fn map_inv(&self, s: &str) -> Result<char, CodeError> {
        match usize::from_str_radix(s, self.mode.radix()) {
            Ok(n) => Alphabet::Ascii128
                .chars()
                .nth(n)
                .ok_or(CodeError::invalid_input_group(s)),
            Err(_) => {
                return Err(CodeError::Input(format!(
                    "error decoding ASCII ({} representation), unable to parse string: {}",
                    self.mode.name(),
                    s
                )))
            }
        }
    }

    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (char, &String)> + '_> {
        let cs = Alphabet::Ascii128.chars();
        match self.mode {
            DisplayMode::EightBitBinary => Box::new(cs.zip(EIGHT_BIT.iter())),
            DisplayMode::SevenBitBinary => Box::new(cs.zip(SEVEN_BIT.iter())),
            DisplayMode::Octal => Box::new(cs.zip(OCTAL.iter())),
            DisplayMode::Decimal => Box::new(cs.zip(DECIMAL.iter())),
            DisplayMode::Hex => Box::new(cs.zip(HEX.iter())),
        }
    }

    pub fn chars_codes_display(&self) -> Box<dyn Iterator<Item = (&&str, &String)> + '_> {
        match self.mode {
            DisplayMode::EightBitBinary => {
                Box::new(CHARACTER_DESCRIPTIONS.iter().zip(EIGHT_BIT.iter()))
            }
            DisplayMode::SevenBitBinary => {
                Box::new(CHARACTER_DESCRIPTIONS.iter().zip(SEVEN_BIT_DISPLAY.iter()))
            }
            DisplayMode::Octal => Box::new(CHARACTER_DESCRIPTIONS.iter().zip(OCTAL.iter())),
            DisplayMode::Decimal => Box::new(CHARACTER_DESCRIPTIONS.iter().zip(DECIMAL.iter())),
            DisplayMode::Hex => Box::new(CHARACTER_DESCRIPTIONS.iter().zip(HEX.iter())),
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

    const PLAINTEXT: &'static str = "0\0␀A ␠";
    const CIPHERTEXT: &'static str = "001100000000000000000000010000010010000000100000";

    #[test]
    fn encrypt_test() {
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
