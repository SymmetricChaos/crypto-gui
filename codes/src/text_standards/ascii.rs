use crate::{errors::CodeError, traits::Code};
use bimap::BiMap;
use itertools::Itertools;
use lazy_static::lazy_static;
use utils::{
    preset_alphabet::Alphabet,
    text_functions::{bimap_from_iter, string_chunks},
};

lazy_static! {
    pub static ref CONTROL_PICTURE_MAP: BiMap<u8, char> = bimap_from_iter(
        (0..33)
            .chain(std::iter::once(127))
            .zip("␀␁␂␃␄␅␆␇␈␉␊␋␌␍␎␏␐␑␒␓␔␕␖␗␘␙␚␛␜␝␞␟␠␡".chars())
    );
}

const CHARACTER_DESCRIPTIONS: [&'static str; 128] = [
    "␀  null",
    "␁  start of heading",
    "␂  start of text",
    "␃  end of text",
    "␄  end of transmission",
    "␅  enquiry",
    "␆  acknowledge",
    "␇  bell",
    "␈  backspace",
    "␉  horizontal tab",
    "␊  line feed",
    "␋  vertical tab",
    "␌  form feed",
    "␍  carriage return",
    "␎  shift out",
    "␏  shift in",
    "␐  data link escape",
    "␑  device control 1",
    "␒  device control 2",
    "␓  device control 3",
    "␔  device control 4",
    "␕  negative acknowledge",
    "␖  synchronous idle",
    "␗  end of transmission block",
    "␘  cancel",
    "␙  end of medium",
    "␚  substitute",
    "␛  escape",
    "␜  file separator",
    "␝  group separator",
    "␞  record separator",
    "␟  uni separator",
    "␠  space",
    "!  exclamation mark",
    "\"  quotation mark",
    "#  number sign",
    "$  dollar sign",
    "%  percent sign",
    "&  ampersand",
    "'  apostrophe",
    "(  left parenthesis",
    ")  right parenthesis",
    "*  asterisk",
    "+  plus sign",
    ",  comma",
    "-  hyphen-minus",
    ".  full stop",
    "/  solidus",
    "0",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    ":  colon",
    ";  semicolon",
    "<  less than sign",
    "=  equals sign",
    ">  greater than sign",
    "?  question mark",
    "@  at sign",
    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
    "G",
    "H",
    "I",
    "J",
    "K",
    "L",
    "M",
    "N",
    "O",
    "P",
    "Q",
    "R",
    "S",
    "T",
    "U",
    "V",
    "W",
    "X",
    "Y",
    "Z",
    "[  left square bracket",
    "\\  reverse solidus",
    "]  right square bracket",
    "^  caret",
    "_  underscore",
    "`  grave accent",
    "a",
    "b",
    "c",
    "d",
    "e",
    "f",
    "g",
    "h",
    "i",
    "j",
    "k",
    "l",
    "m",
    "n",
    "o",
    "p",
    "q",
    "r",
    "s",
    "t",
    "u",
    "v",
    "w",
    "x",
    "y",
    "z",
    "{  left curly bracket",
    "|  vertical line",
    "}  right curly bracket",
    "~  tilde",
    "␡  delete",
];

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
    pub static ref DECIMAL: Vec<String> = (0..128).map(|n| format!("{:03}", n)).collect_vec();
    pub static ref DECIMAL_DISPLAY: Vec<String> =
        (0..128).map(|n| format!(" {:3}", n)).collect_vec();
    pub static ref HEX: Vec<String> = (0..128).map(|n| format!("{:02x}", n)).collect_vec();
}

pub struct Ascii {
    pub mode: DisplayMode,
    pub spaced: bool,
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

impl Default for Ascii {
    fn default() -> Self {
        Ascii {
            mode: DisplayMode::EightBitBinary,
            spaced: false,
        }
    }
}

impl Code for Ascii {
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
mod ascii_tests {
    use super::*;

    const PLAINTEXT: &'static str = "0\0␀A ␠";
    const CIPHERTEXT: &'static str = "001100000000000000000000010000010010000000100000";

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
