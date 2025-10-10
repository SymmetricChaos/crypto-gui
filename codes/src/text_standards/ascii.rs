use crate::traits::Code;
use bimap::BiMap;
use itertools::Itertools;
use utils::{errors::GeneralError, preset_alphabet::Alphabet, text_functions::string_chunks};

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
    "␟  unit separator",
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

crate::lazy_bimap!(
    CONTROL_PICTURE_MAP: BiMap<u8, char> =
        (0..33).chain(std::iter::once(127)).zip("␀␁␂␃␄␅␆␇␈␉␊␋␌␍␎␏␐␑␒␓␔␕␖␗␘␙␚␛␜␝␞␟␠␡".chars())
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpperBit {
    Unset,
    Set,
    Even,
    Odd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    EightBit,
    SevenBit,
    Octal,
    Decimal,
    Hex,
}

impl DisplayMode {
    pub fn radix(&self) -> u32 {
        match self {
            DisplayMode::EightBit => 2,
            DisplayMode::SevenBit => 2,
            DisplayMode::Octal => 8,
            DisplayMode::Decimal => 10,
            DisplayMode::Hex => 16,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            DisplayMode::EightBit => "binary",
            DisplayMode::SevenBit => "binary",
            DisplayMode::Octal => "octal",
            DisplayMode::Decimal => "decimal",
            DisplayMode::Hex => "hexadecimal",
        }
    }

    pub fn width(&self) -> usize {
        match self {
            DisplayMode::EightBit => 8,
            DisplayMode::SevenBit => 7,
            DisplayMode::Octal => 3,
            DisplayMode::Decimal => 3,
            DisplayMode::Hex => 2,
        }
    }
}

// pub static CONTROL_PICTURE_MAP: LazyLock<BiMap<u8, char>> = LazyLock::new(|| {
//     bimap_from_iter(
//         (0..33)
//             .chain(std::iter::once(127))
//             .zip("␀␁␂␃␄␅␆␇␈␉␊␋␌␍␎␏␐␑␒␓␔␕␖␗␘␙␚␛␜␝␞␟␠␡".chars()),
//     )
// });

// lazy_static! {
//     pub static ref CONTROL_PICTURE_MAP: BiMap<u8, char> = bimap_from_iter(
//         (0..33)
//             .chain(std::iter::once(127))
//             .zip("␀␁␂␃␄␅␆␇␈␉␊␋␌␍␎␏␐␑␒␓␔␕␖␗␘␙␚␛␜␝␞␟␠␡".chars()),
//     );
//     pub static ref SEVEN_BIT: Vec<String> = (0..128).map(|n| format!("{:07b}", n)).collect_vec();
//     pub static ref EIGHT_BIT: Vec<String> = (0..128).map(|n| format!("{:08b}", n)).collect_vec();
//     pub static ref OCTAL: Vec<String> = (0..128).map(|n| format!("{:03o}", n)).collect_vec();
//     pub static ref DECIMAL: Vec<String> = (0..128).map(|n| format!("{:03}", n)).collect_vec();
//     pub static ref HEX: Vec<String> = (0..128).map(|n| format!("{:02x}", n)).collect_vec();
//     pub static ref ASCII_U8: Vec<u8> = (0..128).collect_vec();
//     pub static ref ASCII_U8_HIGH_BIT_SET: Vec<u8> = (128..=255).collect_vec();
//     pub static ref ASCII_U8_EVEN_PARITY: Vec<u8> = (0..128_u8)
//         .map(|n| if n.count_ones() % 2 == 1 { n } else { n + 128 })
//         .collect_vec();
//     pub static ref ASCII_U8_ODD_PARITY: Vec<u8> = (0..128_u8)
//         .map(|n| if n.count_ones() % 2 == 0 { n } else { n + 128 })
//         .collect_vec();
// }

pub struct Ascii {
    pub mode: DisplayMode,
    pub spaced: bool,
    pub upper_bit: UpperBit,
}

impl Default for Ascii {
    fn default() -> Self {
        Ascii {
            mode: DisplayMode::EightBit,
            spaced: false,
            upper_bit: UpperBit::Unset,
        }
    }
}

impl Ascii {
    fn transform(&self, n: u8) -> String {
        match self.mode {
            DisplayMode::EightBit => format!("{:08b}", n),
            DisplayMode::SevenBit => format!("{:07b}", n),
            DisplayMode::Octal => format!("{:03o}", n),
            DisplayMode::Decimal => format!("{:03}", n),
            DisplayMode::Hex => format!("{:02x}", n),
        }
    }

    fn change_upper_bit(&self, n: u8) -> u8 {
        match self.upper_bit {
            UpperBit::Unset => n,
            UpperBit::Set => n | 0b1000_0000,
            UpperBit::Even => {
                if n.count_ones() % 2 == 1 {
                    n | 0b1000_0000
                } else {
                    n
                }
            }
            UpperBit::Odd => {
                if n.count_ones() % 2 == 0 {
                    n | 0b1000_0000
                } else {
                    n
                }
            }
        }
    }

    pub fn map(&self, c: char) -> Result<String, GeneralError> {
        if c.is_ascii() {
            if self.mode == DisplayMode::SevenBit {
                return Ok(self.transform(c as u8));
            }
            let n = self.change_upper_bit(c as u8);
            return Ok(self.transform(n));
        }
        if let Some(control_val) = CONTROL_PICTURE_MAP.get_by_right(&c) {
            if self.mode == DisplayMode::SevenBit {
                return Ok(self.transform(*control_val));
            }
            let n = self.change_upper_bit(*control_val);
            return Ok(self.transform(n));
        } else {
            Err(GeneralError::invalid_input_char(c))
        }
    }

    pub fn map_inv(&self, s: &str) -> Result<char, GeneralError> {
        if let Ok(n) = usize::from_str_radix(s, self.mode.radix()) {
            if self.mode == DisplayMode::SevenBit {
                return Alphabet::Ascii128
                    .chars()
                    .nth(n)
                    .ok_or(GeneralError::invalid_input_group(s));
            }
            match self.upper_bit {
                UpperBit::Set | UpperBit::Unset => Alphabet::Ascii128
                    .chars()
                    .nth(n % 128)
                    .ok_or(GeneralError::invalid_input_group(s)),
                UpperBit::Even => {
                    if n.count_ones() % 2 == 0 {
                        Alphabet::Ascii128
                            .chars()
                            .nth(n % 128)
                            .ok_or(GeneralError::invalid_input_group(s))
                    } else {
                        Ok('�')
                    }
                }
                UpperBit::Odd => {
                    if n.count_ones() % 2 == 1 {
                        Alphabet::Ascii128
                            .chars()
                            .nth(n % 128)
                            .ok_or(GeneralError::invalid_input_group(s))
                    } else {
                        Ok('�')
                    }
                }
            }
        } else {
            return Err(GeneralError::input(format!(
                "error decoding ASCII ({} representation), unable to parse string: {}",
                self.mode.name(),
                s
            )));
        }
    }

    // pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (char, &String)> + '_> {
    //     let cs = Alphabet::Ascii128.chars();
    //     match self.mode {
    //         DisplayMode::EightBit => Box::new(cs.zip(EIGHT_BIT.iter())),
    //         DisplayMode::SevenBit => Box::new(cs.zip(SEVEN_BIT.iter())),
    //         DisplayMode::Octal => Box::new(cs.zip(OCTAL.iter())),
    //         DisplayMode::Decimal => Box::new(cs.zip(DECIMAL.iter())),
    //         DisplayMode::Hex => Box::new(cs.zip(HEX.iter())),
    //     }
    // }

    pub fn chars_codes_display(&self) -> Box<dyn Iterator<Item = (&&str, String)> + '_> {
        Box::new(
            CHARACTER_DESCRIPTIONS
                .iter()
                .zip((0..128).map(|n| self.transform(self.change_upper_bit(n)))),
        )
    }
}

impl Code for Ascii {
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
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

    fn decode(&self, text: &str) -> Result<String, GeneralError> {
        let chunks = string_chunks(&text.replace(' ', ""), self.mode.width());
        let mut out = String::with_capacity(chunks.len());

        for chunk in chunks {
            // The control pictures have the last same eight bits as the actual control characters.
            // Casting to u8 is guaranteed keep only the last eight bits so it automatically maps control pictures to control characters.
            out.push((self.map_inv(&chunk)? as u8) as char);
        }

        Ok(out)
    }
}

#[cfg(test)]
mod ascii_tests {
    use super::*;

    const PTEXT: &'static str = "0\0␀A ␠";
    const CODETEXT_SEVEN_SP: &'static str = "0110000 0000000 0000000 1000001 0100000 0100000";
    const CODETEXT_EIGHT_SP_EVEN: &'static str =
        "00110000 00000000 00000000 01000001 10100000 10100000";
    const CODETEXT_EIGHT_SP_ODD: &'static str =
        "10110000 10000000 10000000 11000001 00100000 00100000";

    #[test]
    fn encode_test() {
        let mut code = Ascii::default();
        code.mode = DisplayMode::SevenBit;
        code.spaced = true;
        assert_eq!(code.encode(PTEXT).unwrap(), CODETEXT_SEVEN_SP);
        code.mode = DisplayMode::EightBit;
        code.upper_bit = UpperBit::Even;
        assert_eq!(code.encode(PTEXT).unwrap(), CODETEXT_EIGHT_SP_EVEN);
        code.upper_bit = UpperBit::Odd;
        assert_eq!(code.encode(PTEXT).unwrap(), CODETEXT_EIGHT_SP_ODD);
    }

    #[test]
    fn encode_decode_test() {
        let mut code = Ascii::default();
        const GIVEN_TEXT: &'static str = "The quick␠brown fox!␀␀␀Jumps over the lazy(dog)";
        const DECODED_TEXT: &'static str = "The quick brown fox!␀␀␀Jumps over the lazy(dog)";

        code.upper_bit = UpperBit::Set;

        for mode in [
            DisplayMode::EightBit,
            DisplayMode::SevenBit,
            DisplayMode::Octal,
            DisplayMode::Decimal,
            DisplayMode::Hex,
        ] {
            code.mode = mode;
            for upper_bit in [
                UpperBit::Even,
                UpperBit::Odd,
                UpperBit::Set,
                UpperBit::Unset,
            ] {
                code.upper_bit = upper_bit;
                let encoded = code
                    .encode(GIVEN_TEXT)
                    .expect(&format!("encoding ASCII {:?} {:?} error", mode, upper_bit));
                let decoded = code
                    .decode(&encoded)
                    .expect(&format!("decoding ASCII {:?} {:?} error", mode, upper_bit));
                if decoded != DECODED_TEXT {
                    panic!(
                        "decoded ASCII {:?} {:?} not equivalent to plaintext\n{}",
                        mode, upper_bit, decoded
                    )
                }
            }
        }
    }
}
