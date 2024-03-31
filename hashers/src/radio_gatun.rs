use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

use crate::{errors::HasherError, traits::ClassicHasher};

// https://radiogatun.noekeon.org/
// https://en.wikibooks.org/wiki/Cryptography/RadioGat%C3%BAn
pub struct RadioGatun {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub hash_len: u32,
}

impl Default for RadioGatun {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            hash_len: 32,
        }
    }
}

impl RadioGatun {
    const ROTATION_32: [u32; 19] = [
        0, 1, 3, 6, 10, 15, 21, 28, 4, 13, 23, 2, 14, 27, 9, 24, 8, 25, 11,
    ];
    // const ROTATION_64: [u32; 19] = [
    //     0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 2, 14, 27, 41, 56, 8, 25, 43,
    // ];

    // XOR words from the mill into the belt
    fn belt_to_mill_feedforward(belt_words: &mut [u32], mill_words: &[u32]) {
        for i in 0..12 {
            belt_words[i + ((i % 3) * 13)] ^= mill_words[i + 1]
        }
    }

    fn mill(mill_words: &mut [u32]) {
        // main mill operation
        let mut temp_arr = [0; 19];
        for a in 0..19 {
            let r = Self::ROTATION_32[a];
            let i = (a * 7) % 19;
            let mut k = mill_words[i];
            k = k ^ ((mill_words[(i as usize + 1) % 19]) | (!mill_words[(i as usize + 2) % 19]));
            temp_arr[a] = k.rotate_right(r)
        }
        // update the mill
        for i in 0..19 {
            mill_words[i] = temp_arr[i] ^ temp_arr[(i + 1) % 19] ^ temp_arr[(i + 4) % 19]
        }
    }

    fn rotate_belt(belt_words: &mut [u32]) {
        belt_words[0..13].rotate_right(1);
        belt_words[13..26].rotate_right(1);
        belt_words[26..39].rotate_right(1);
    }

    fn iota(mill_words: &mut [u32]) {
        mill_words[0] ^= 1;
    }

    fn belt_to_mill(belt_words: &[u32], mill_words: &mut [u32]) {
        mill_words[13] ^= belt_words[0];
        mill_words[14] ^= belt_words[13];
        mill_words[15] ^= belt_words[26];
    }

    pub fn beltmill(belt_words: &mut [u32], mill_words: &mut [u32]) {
        Self::belt_to_mill_feedforward(belt_words, mill_words);
        Self::rotate_belt(belt_words);
        Self::mill(mill_words);
        Self::iota(mill_words);
        Self::belt_to_mill(belt_words, mill_words);
    }

    // pub fn print_state(belt_words: &[u32], mill_words: &[u32]) {
    // println!("{:08x?}", mill_words);
    // println!("{:08x?}", &belt_words[0..13]);
    // println!("{:08x?}", &belt_words[13..26]);
    // println!("{:08x?}\n\n", &belt_words[26..39]);
    // }
}

impl ClassicHasher for RadioGatun {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();
        input.push(0x01);
        while input.len() % 12 != 0 {
            input.push(0x00)
        }

        let words = input
            .chunks_exact(4)
            .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
            .collect_vec();

        let mut belt = [0_u32; 39]; // three rows of 13 words
        let mut mill = [0_u32; 19]; // one row of 19 words

        for words in words.chunks_exact(3) {
            belt[0] ^= words[0];
            mill[16] ^= words[0];

            belt[13] ^= words[1];
            mill[17] ^= words[1];

            belt[26] ^= words[2];
            belt[18] ^= words[2];

            for _ in 0..18 {
                Self::beltmill(&mut belt, &mut mill);
            }
            // Self::print_state(&belt, &mill);
        }

        let mut out = Vec::new();
        while out.len() < self.hash_len as usize {
            out.extend_from_slice(&mill[1].to_le_bytes());
            out.extend_from_slice(&mill[2].to_le_bytes());
            Self::beltmill(&mut belt, &mut mill);
        }
        out.truncate(self.hash_len as usize);
        out
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| HasherError::general("byte format error"))?;
        let out = self.hash(&mut bytes);
        Ok(self.output_format.byte_slice_to_text(&out))
    }
}

#[cfg(test)]
mod radio_gatun_tests {
    use super::*;

    #[test]
    fn test_suite() {
        let mut hasher = RadioGatun::default();
        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;
        assert_eq!(
            "9ebdd24f469993796c4aac6a821735a65a3cdef8a359944ce71f34e7a08e1182",
            hasher.hash_bytes_from_string("1234").unwrap()
        );
    }
}
