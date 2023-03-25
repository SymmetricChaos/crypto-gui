use crate::{errors::Error, text_aux::PresetAlphabet::Ascii128};
use itertools::Itertools;
use lazy_static::lazy_static;

use super::Code;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    EightBitBinary,
    SevenBitBinary,
    Octal,
    Decimal,
    Hex,
}

impl DisplayMode {
    pub fn radix(&self) -> u32 {
        match self {
            DisplayMode::EightBitBinary => 2,
            DisplayMode::SevenBitBinary => 2,
            DisplayMode::Octal => 8,
            DisplayMode::Decimal => 10,
            DisplayMode::Hex => 16,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            DisplayMode::EightBitBinary => "binary",
            DisplayMode::SevenBitBinary => "binary",
            DisplayMode::Octal => "octal",
            DisplayMode::Decimal => "decimal",
            DisplayMode::Hex => "hexadecimal",
        }
    }

    pub fn width(&self) -> usize {
        match self {
            DisplayMode::EightBitBinary => 8,
            DisplayMode::SevenBitBinary => 7,
            DisplayMode::Octal => 3,
            DisplayMode::Decimal => 3,
            DisplayMode::Hex => 2,
        }
    }
}

lazy_static! {
    pub static ref SEVEN_BIT: Vec<String> = (0..128).map(|n| format!("{:07b}", n)).collect_vec();
    pub static ref SEVEN_BIT_DISPLAY: Vec<String> =
        (0..128).map(|n| format!(" {:07b}", n)).collect_vec();
    pub static ref EIGHT_BIT: Vec<String> = (0..128).map(|n| format!("{:08b}", n)).collect_vec();
    pub static ref OCTAL: Vec<String> = (0..128).map(|n| format!("{:03o}", n)).collect_vec();
    pub static ref DECIMAL: Vec<String> = (0..128).map(|n| format!("{:3}", n)).collect_vec();
    pub static ref HEX: Vec<String> = (0..128).map(|n| format!("{:02x}", n)).collect_vec();
}

pub struct Ascii {
    pub mode: DisplayMode,
}

impl Ascii {
    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (char, &String)> + '_> {
        let cs = Ascii128.chars();
        match self.mode {
            DisplayMode::EightBitBinary => Box::new(cs.zip(EIGHT_BIT.iter())),
            DisplayMode::SevenBitBinary => Box::new(cs.zip(SEVEN_BIT.iter())),
            DisplayMode::Octal => Box::new(cs.zip(OCTAL.iter())),
            DisplayMode::Decimal => Box::new(cs.zip(DECIMAL.iter())),
            DisplayMode::Hex => Box::new(cs.zip(HEX.iter())),
        }
    }

    pub fn chars_codes_display(&self) -> Box<dyn Iterator<Item = (char, &String)> + '_> {
        let cs = Ascii128.chars();
        match self.mode {
            DisplayMode::EightBitBinary => Box::new(cs.zip(EIGHT_BIT.iter())),
            DisplayMode::SevenBitBinary => Box::new(cs.zip(SEVEN_BIT_DISPLAY.iter())),
            DisplayMode::Octal => Box::new(cs.zip(OCTAL.iter())),
            DisplayMode::Decimal => Box::new(cs.zip(DECIMAL.iter())),
            DisplayMode::Hex => Box::new(cs.zip(HEX.iter())),
        }
    }
}

impl Default for Ascii {
    fn default() -> Self {
        Ascii {
            mode: DisplayMode::EightBitBinary,
        }
    }
}

impl Code for Ascii {
    fn encode(&self, text: &str) -> Result<String, Error> {
        if !text.is_ascii() {
            return Err(Error::Input("text includes non-ASCII characters".into()));
        }
        let chunks = text.bytes();
        let s: String = match self.mode {
            DisplayMode::EightBitBinary => chunks.map(|n| (format!("{:08b}", n))).join(" "),
            DisplayMode::SevenBitBinary => chunks.map(|n| (format!("{:07b}", n))).join(" "),
            DisplayMode::Octal => chunks.map(|n| (format!("{:04o}", n))).join(" "),
            DisplayMode::Decimal => chunks.map(|n| (format!("{}", n))).join(" "),
            DisplayMode::Hex => chunks.map(|n| (format!("{:02x}", n))).join(" "),
        };
        Ok(s)
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let chunks = text.split(" ");
        let radix = self.mode.radix();
        let mut vec = Vec::with_capacity(chunks.clone().count());

        for chunk in chunks {
            match u8::from_str_radix(chunk, radix) {
                Ok(n) => vec.push(n),
                Err(_) => {
                    return Err(Error::Input(format!(
                        "error decoding ASCII ({} representation), unable to parse string: {}",
                        self.mode.name(),
                        chunk
                    )))
                }
            }
        }

        String::from_utf8(vec).map_err(|e| Error::Input(e.to_string()))
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod ascii_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "01010100 01001000 01000101 01010001 01010101 01001001 01000011 01001011 01000010 01010010 01001111 01010111 01001110 01000110 01001111 01011000 01001010 01010101 01001101 01010000 01010011 01001111 01010110 01000101 01010010 01010100 01001000 01000101 01001100 01000001 01011010 01011001 01000100 01001111 01000111";

    #[test]
    fn encrypt_test() {
        let code = Ascii::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = Ascii::default();
        assert_eq!(code.decode(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}

#[test]
fn encrypt_decrypt() {
    let mut code = Ascii::default();
    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";

    for mode in [
        DisplayMode::EightBitBinary,
        DisplayMode::SevenBitBinary,
        DisplayMode::Octal,
        DisplayMode::Decimal,
        DisplayMode::Hex,
    ] {
        code.mode = mode;
        let encoded = code
            .encode(PLAINTEXT)
            .expect(&format!("encoding ASCII {:?} error", mode));
        let decoded = code
            .decode(&encoded)
            .expect(&format!("decoding ASCII {:?} error", mode));
        if decoded != PLAINTEXT {
            panic!("decoded ASCII {:?} not equivalent to plaintext", mode)
        }
    }
}
