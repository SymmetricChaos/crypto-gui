use super::braille_data::{ASCII_ORDER, UNICODE_ORDER};
use crate::{braille::braille_data::UEB_ORDER, errors::CodeError, traits::Code};
use bimap::BiMap;

// All of these are in UEB order
const BRAILLE_DOTS: [&'static str; 64] = [
    "0", "1", "12", "14", "145", "15", "124", "1245", "125", "24", "245", "13", "123", "143",
    "1453", "153", "1243", "12345", "1235", "234", "2345", "136", "1236", "1436", "14536", "1536",
    "12436", "123456", "12356", "2346", "23456", "16", "126", "146", "1456", "156", "1246",
    "12456", "1256", "246", "2456", "2", "23", "25", "256", "26", "235", "2356", "236", "35",
    "356", "34", "346", "3456", "345", "3", "36", "4", "45", "456", "5", "46", "56", "6",
];

const BRAILLE_BITS: [&'static str; 64] = [
    "000000", "100000", "110000", "100100", "100110", "100010", "110100", "110110", "110010",
    "010100", "010110", "101000", "111000", "101100", "101110", "101010", "111100", "111110",
    "111010", "011100", "011110", "101001", "111001", "101101", "101111", "101011", "111101",
    "111111", "111011", "011101", "011111", "100001", "110001", "100101", "100111", "100011",
    "110101", "110111", "110011", "010101", "010111", "010000", "011000", "010010", "010011",
    "010001", "011010", "011011", "011001", "001010", "001011", "001100", "001101", "001111",
    "001110", "001000", "001001", "000100", "000110", "000111", "000010", "000101", "000011",
    "000001",
];

// Offsets from 2800 (hex)
const BRAILLE_HEX: [&'static str; 64] = [
    "00", "01", "03", "09", "19", "11", "0B", "1B", "13", "0A", "1A", "05", "07", "0D", "1D", "15",
    "0F", "1F", "17", "0E", "1E", "25", "27", "2D", "3D", "35", "2F", "3F", "37", "2E", "3E", "21",
    "23", "29", "39", "31", "2B", "3B", "33", "2A", "3A", "02", "06", "12", "32", "22", "16", "36",
    "26", "14", "34", "0C", "2C", "3C", "1C", "04", "24", "08", "18", "38", "10", "28", "30", "20",
];

// Offsets from 10240 (decimal)
// const BRAILLE_OFFSETS: [u32; 64] = [
//     0, 1, 3, 9, 25, 17, 11, 27, 19, 10, 26, 5, 7, 13, 29, 21, 15, 31, 23, 14, 30, 37, 39, 45, 61,
//     53, 47, 63, 55, 46, 62, 33, 35, 41, 57, 49, 43, 59, 51, 42, 58, 2, 6, 18, 50, 34, 22, 54, 38,
//     20, 52, 12, 44, 60, 28, 4, 36, 8, 24, 56, 16, 40, 48, 32,
// ];

const BRAILLE_ASCII: [&'static str; 64] = [
    " ", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
    "S", "T", "U", "V", "X", "Y", "Z", "&", "=", "(", "!", ")", "*", "<", "%", "?", ":", "$", "]",
    "\\", "[", "W", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "/", "+", "#", ">", "'", "-",
    "@", "^", "_", "\"", ".", ";", ",",
];

crate::lazy_bimap!(
    BRAILLE_DOTS_MAP: BiMap<char, &'static str> = UEB_ORDER.chars().zip(BRAILLE_DOTS.into_iter());
    BRAILLE_BITS_MAP: BiMap<char, &'static str> = UEB_ORDER.chars().zip(BRAILLE_BITS.into_iter());
    BRAILLE_HEX_MAP: BiMap<char, &'static str> = UEB_ORDER.chars().zip(BRAILLE_HEX.into_iter());
    BRAILLE_ASCII_MAP: BiMap<char, &'static str> = UEB_ORDER.chars().zip(BRAILLE_ASCII.into_iter());
);

#[derive(Debug, PartialEq, Eq)]
pub enum BrailleEncodingType {
    Dots,
    Bits,
    Hex,
    Ascii,
}

impl BrailleEncodingType {
    pub fn encode(&self, c: char) -> Option<&str> {
        match self {
            Self::Dots => BRAILLE_DOTS_MAP.get_by_left(&c),
            Self::Bits => BRAILLE_BITS_MAP.get_by_left(&c),
            Self::Hex => BRAILLE_HEX_MAP.get_by_left(&c),
            Self::Ascii => BRAILLE_ASCII_MAP.get_by_left(&c),
        }
        .copied()
    }

    pub fn decode(&self, s: &str) -> Option<char> {
        match self {
            Self::Dots => BRAILLE_DOTS_MAP.get_by_right(s),
            Self::Bits => BRAILLE_BITS_MAP.get_by_right(s),
            Self::Hex => BRAILLE_HEX_MAP.get_by_right(s),
            Self::Ascii => BRAILLE_ASCII_MAP.get_by_right(s),
        }
        .copied()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BrailleOrder {
    Ueb,
    Unicode,
    Ascii,
}

pub struct BrailleEncoding {
    pub order: BrailleOrder,
    pub mode: BrailleEncodingType,
}

impl Default for BrailleEncoding {
    fn default() -> Self {
        Self {
            order: BrailleOrder::Ueb,
            mode: BrailleEncodingType::Dots,
        }
    }
}

impl BrailleEncoding {
    pub fn chars_codes(&self) -> impl Iterator<Item = (char, &str)> {
        let cs = match self.order {
            BrailleOrder::Ueb => UEB_ORDER.chars(),
            BrailleOrder::Unicode => UNICODE_ORDER.chars(),
            BrailleOrder::Ascii => ASCII_ORDER.chars(),
        };
        cs.map(|c| (c, self.mode.encode(c).unwrap()))
    }
}

impl Code for BrailleEncoding {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();

        for c in text.chars() {
            if c.is_whitespace() {
                out.push(c);
            } else {
                out.push_str(
                    self.mode
                        .encode(c)
                        .ok_or_else(|| CodeError::invalid_input_char(c))?,
                );
            }
            if self.mode != BrailleEncodingType::Ascii {
                out.push(' ');
            }
        }
        if self.mode != BrailleEncodingType::Ascii {
            out.pop();
        }

        Ok(out)
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();

        if self.mode == BrailleEncodingType::Ascii {
            // Commonly BrailleASCII values are given with lowercase letters so that is fixed here
            for c in text.to_ascii_uppercase().chars() {
                if c.is_whitespace() {
                    out.push(c);
                } else {
                    out.push(
                        self.mode
                            .decode(&c.to_string())
                            .ok_or_else(|| CodeError::invalid_input_char(c))?,
                    );
                }
            }
        } else {
            for s in text.split(" ") {
                out.push(
                    self.mode
                        .decode(s)
                        .ok_or_else(|| CodeError::invalid_input_group(s))?,
                );
            }
        }

        Ok(out)
    }
}

#[cfg(test)]
mod braille_ascii_tests {

    use crate::braille::braille_data::UEB_ORDER;

    use super::*;

    const BRAILLE_TEXT: &'static str = "⠀⠮⠐⠼⠫⠩⠯⠄⠷";
    const ASCII_TEXT: &'static str = " !\"#$%&'(";
    const BITS_TEXT: &'static str =
        "000000 011101 000010 001111 110101 100101 111101 001000 111011";
    const DOTS_TEXT: &'static str = "0 2346 5 3456 1246 146 12436 3 12356";
    const HEX_TEXT: &'static str = "00 2E 10 3C 2B 29 2F 04 37";

    #[test]
    fn encode() {
        let mut code = BrailleEncoding::default();
        for (mode, text) in [
            (BrailleEncodingType::Ascii, ASCII_TEXT),
            (BrailleEncodingType::Bits, BITS_TEXT),
            (BrailleEncodingType::Dots, DOTS_TEXT),
            (BrailleEncodingType::Hex, HEX_TEXT),
        ] {
            code.mode = mode;
            assert_eq!(text, code.encode(BRAILLE_TEXT).unwrap())
        }
    }

    #[test]
    fn decode() {
        let mut code = BrailleEncoding::default();
        for (mode, text) in [
            (BrailleEncodingType::Ascii, ASCII_TEXT),
            (BrailleEncodingType::Bits, BITS_TEXT),
            (BrailleEncodingType::Dots, DOTS_TEXT),
            (BrailleEncodingType::Hex, HEX_TEXT),
        ] {
            code.mode = mode;
            assert_eq!(BRAILLE_TEXT, code.decode(text).unwrap())
        }
    }

    #[test]
    #[ignore = "pairing test"]
    fn pairing() {
        for c in UEB_ORDER.chars() {
            println!("{} {}", c, BRAILLE_DOTS_MAP.get_by_left(&c).unwrap())
        }
    }

    #[test]
    #[ignore = "create alternatives"]
    fn create() {
        // println!("create bits");
        // let mut output: Vec<String> = Vec::new();
        // for s in BRAILLE_DOTS.into_iter() {
        //     let mut bits = ['0'; 6];
        //     for c in s.chars() {
        //         if c == '0' {
        //             continue;
        //         }
        //         bits[c.to_digit(10).unwrap() as usize - 1] = '1';
        //     }
        //     output.push(bits.into_iter().collect())
        // }
        // println!("{:?}", output);

        let mut hex_values: Vec<String> = Vec::new();
        let mut braille_nums = Vec::new();
        let values = [1, 2, 4, 8, 16, 32];
        for s in BRAILLE_BITS.into_iter() {
            let mut v: u32 = 0;
            for (pos, c) in s.chars().enumerate() {
                if c == '1' {
                    v += values[pos]
                };
            }
            braille_nums.push(v);
            hex_values.push(format!("{v:02X}"))
        }
        println!("{:?}", hex_values);
        println!("{:?}", braille_nums);
    }
}
