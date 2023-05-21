use crate::{errors::CodeError, traits::Code};
use bimap::BiMap;
use itertools::Itertools;
use lazy_static::lazy_static;
use utils::{functions::bimap_from_iter, preset_alphabet::PresetAlphabet};

lazy_static! {
    pub static ref CONTROL_PICTURE_MAP: BiMap<u8, char> = bimap_from_iter(
        (0..33)
            .chain(std::iter::once(127))
            .zip("␀␁␂␃␄␅␆␇␈␉␊␋␌␍␎␏␐␑␒␓␔␕␖␗␘␙␚␛␜␝␞␟␠␡".chars())
    );
}

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
    pub static ref DECIMAL: Vec<String> = (0..128).map(|n| format!("{}", n)).collect_vec();
    pub static ref DECIMAL_DISPLAY: Vec<String> =
        (0..128).map(|n| format!(" {:3}", n)).collect_vec();
    pub static ref HEX: Vec<String> = (0..128).map(|n| format!("{:02x}", n)).collect_vec();
}

pub struct Ascii {
    pub mode: DisplayMode,
}

impl Ascii {
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
        let radix = self.mode.radix();
        match usize::from_str_radix(s, radix) {
            Ok(n) => PresetAlphabet::Ascii128
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
        let cs = PresetAlphabet::Ascii128.chars();
        match self.mode {
            DisplayMode::EightBitBinary => Box::new(cs.zip(EIGHT_BIT.iter())),
            DisplayMode::SevenBitBinary => Box::new(cs.zip(SEVEN_BIT.iter())),
            DisplayMode::Octal => Box::new(cs.zip(OCTAL.iter())),
            DisplayMode::Decimal => Box::new(cs.zip(DECIMAL.iter())),
            DisplayMode::Hex => Box::new(cs.zip(HEX.iter())),
        }
    }

    pub fn chars_codes_display(&self) -> Box<dyn Iterator<Item = (char, &String)> + '_> {
        let cs = "␀␁␂␃␄␅␆␇␈␉␊␋␌␍␎␏␐␑␒␓␔␕␖␗␘␙␚␛␜␝␞␟␠!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~␡".chars();
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
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::new();
        for c in text.chars() {
            out.push(self.map(c)?)
        }
        Ok(out.into_iter().join(" "))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let chunks = text.split(" ");
        let mut out = String::with_capacity(chunks.clone().count());

        for chunk in chunks {
            out.push(self.map_inv(chunk)?)
        }

        Ok(out)
    }
}

#[cfg(test)]
mod ascii_tests {
    use super::*;

    const PLAINTEXT: &'static str = "0\0␀A ␠";
    const CIPHERTEXT: &'static str = "00110000 00000000 00000000 01000001 00100000 00100000";

    #[test]
    fn encrypt_test() {
        let code = Ascii::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn encrypt_decrypt_test() {
        let mut code = Ascii::default();
        const GIVEN_TEXT: &'static str = "The quick␠brown fox!␀␀␀Jumps over the lazy(dog)";
        const DECODED_TEXT: &'static str = "The quick brown fox!␀␀␀Jumps over the lazy(dog)";

        for mode in [
            DisplayMode::EightBitBinary,
            DisplayMode::SevenBitBinary,
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
