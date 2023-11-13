use bimap::BiMap;
use lazy_static::lazy_static;

use crate::{braille::braille_data::UEB_ORDER, errors::CodeError, traits::Code};
use utils::text_functions::bimap_from_iter;

use super::braille_data::UNICODE_ORDER;

const BRAILLE_DOTS: [&'static str; 63] = [
    "1", "12", "14", "145", "15", "124", "1245", "125", "24", "245", "13", "123", "143", "1453",
    "153", "1243", "12345", "1235", "234", "2345", "136", "1236", "1436", "14536", "1536", "12436",
    "123456", "12356", "2346", "23456", "16", "126", "146", "1456", "156", "1246", "12456", "1256",
    "246", "2456", "2", "23", "25", "256", "26", "235", "2356", "236", "35", "356", "34", "346",
    "3456", "345", "3", "36", "4", "45", "456", "5", "46", "56", "6",
];

const BRAILLE_BITS: [&'static str; 63] = [
    "100000", "110000", "100100", "100110", "100010", "110100", "110110", "110010", "010100",
    "010110", "101000", "111000", "101100", "101110", "101010", "111100", "111110", "111010",
    "011100", "011110", "101001", "111001", "101101", "101111", "101011", "111101", "111111",
    "111011", "011101", "011111", "100001", "110001", "100101", "100111", "100011", "110101",
    "110111", "110011", "010101", "010111", "010000", "011000", "010010", "010011", "010001",
    "011010", "011011", "011001", "001010", "001011", "001100", "001101", "001111", "001110",
    "001000", "001001", "000100", "000110", "000111", "000010", "000101", "000011", "000001",
];

// Offsets from 2800 (hex)
const BRAILLE_HEX: [&'static str; 63] = [
    "01", "03", "09", "13", "0B", "0B", "15", "0D", "0A", "14", "05", "07", "0D", "17", "0F", "0F",
    "19", "11", "0E", "18", "19", "1B", "21", "2B", "23", "23", "2D", "25", "22", "2C", "15", "17",
    "1D", "27", "1F", "1F", "29", "21", "1E", "28", "02", "06", "0C", "20", "16", "10", "24", "1A",
    "0E", "22", "0C", "20", "2A", "16", "04", "18", "08", "12", "26", "0A", "1C", "1E", "14",
];

// Offsets from 10240 (decimal)
const BRAILLE_OFFSETS: [u32; 63] = [
    1, 3, 9, 25, 17, 11, 27, 19, 10, 26, 5, 7, 13, 29, 21, 15, 31, 23, 14, 30, 37, 39, 45, 61, 53,
    47, 63, 55, 46, 62, 33, 35, 41, 57, 49, 43, 59, 51, 42, 58, 2, 6, 18, 50, 34, 22, 54, 38, 20,
    52, 12, 44, 60, 28, 4, 36, 8, 24, 56, 16, 40, 48, 32,
];

lazy_static! {
    pub static ref BRAILLE_DOTS_MAP: BiMap<char, &'static str> =
        bimap_from_iter(UEB_ORDER.chars().zip(BRAILLE_DOTS.into_iter()));
    pub static ref BRAILLE_BITS_MAP: BiMap<char, &'static str> =
        bimap_from_iter(UEB_ORDER.chars().zip(BRAILLE_BITS.into_iter()));
    pub static ref BRAILLE_HEX_MAP: BiMap<char, &'static str> =
        bimap_from_iter(UEB_ORDER.chars().zip(BRAILLE_HEX.into_iter()));
    pub static ref BRAILLE_OFFSET_MAP: BiMap<char, u32> =
        bimap_from_iter(UEB_ORDER.chars().zip(BRAILLE_OFFSETS.into_iter()));
}

pub enum BrailleNumberingMode {
    Dots,
    Bits,
    Hex,
}

impl BrailleNumberingMode {
    pub fn encode(&self, c: char) -> Option<&str> {
        match self {
            Self::Dots => BRAILLE_DOTS_MAP.get_by_left(&c).copied(),
            Self::Bits => BRAILLE_BITS_MAP.get_by_left(&c).copied(),
            Self::Hex => BRAILLE_HEX_MAP.get_by_left(&c).copied(),
        }
    }

    pub fn decode(&self, s: &str) -> Option<&char> {
        match self {
            Self::Dots => BRAILLE_DOTS_MAP.get_by_right(s),
            Self::Bits => BRAILLE_BITS_MAP.get_by_right(s),
            Self::Hex => BRAILLE_HEX_MAP.get_by_right(s),
        }
    }
}

pub struct BrailleNumbering {
    ueb_order: bool,
    mode: BrailleNumberingMode,
}

impl Default for BrailleNumbering {
    fn default() -> Self {
        Self {
            ueb_order: true,
            mode: BrailleNumberingMode::Dots,
        }
    }
}

impl BrailleNumbering {
    pub fn chars_codes(&self) -> impl Iterator<Item = (char, &str)> {
        let cs = if self.ueb_order {
            UEB_ORDER.chars()
        } else {
            UNICODE_ORDER.chars()
        };
        cs.map(|c| (c, self.mode.encode(c).unwrap()))
    }
}

// impl Code for BrailleNumbering {
//     fn encode(&self, text: &str) -> Result<String, CodeError> {
//         let mut out = String::new();

//         for c in text.chars() {
//             if c.is_whitespace() {
//                 out.push(c);
//                 continue;
//             }

//             out.push(
//                 *ASCII_MAP
//                     .get_by_left(&c.to_ascii_uppercase())
//                     .ok_or_else(|| CodeError::invalid_input_char(c))?,
//             );
//         }

//         Ok(out)
//     }

//     fn decode(&self, text: &str) -> Result<String, CodeError> {
//         let mut out = String::new();

//         for c in text.chars() {
//             if c.is_whitespace() {
//                 out.push(c);
//                 continue;
//             }
//             out.push(
//                 *ASCII_MAP
//                     .get_by_right(&c)
//                     .ok_or_else(|| CodeError::invalid_input_char(c))?,
//             );
//         }

//         Ok(out)
//     }
// }

#[cfg(test)]
mod braille_ascii_tests {
    use crate::braille::braille_data::UEB_ORDER;

    use super::*;

    #[test]
    #[ignore = "pairing test"]
    fn pairing() {
        for c in UEB_ORDER.chars() {
            println!("{} {}", c, BRAILLE_DOTS_MAP.get_by_left(&c).unwrap())
        }
        for n in 1..64 {
            print!("{}", BRAILLE_OFFSET_MAP.get_by_right(&n).unwrap());
        }
        println!("")
    }

    #[test]
    #[ignore = "create alternatives"]
    fn create() {
        println!("create bits");
        let mut output: Vec<String> = Vec::new();
        for s in BRAILLE_DOTS.into_iter() {
            let mut bits = ['0'; 6];
            for c in s.chars() {
                bits[c.to_digit(10).unwrap() as usize - 1] = '1';
            }
            output.push(bits.into_iter().collect())
        }
        println!("{:?}", output);

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
            hex_values.push(format!("{v:02x}"))
        }
        println!("{:?}", hex_values);
        println!("{:?}", braille_nums);
    }
}
