use utils::byte_formatting::ByteFormat;

use crate::{errors::HasherError, traits::ClassicHasher};

// https://radiogatun.noekeon.org/
// https://en.wikibooks.org/wiki/Cryptography/RadioGat%C3%BAn
pub struct RadioGatun {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for RadioGatun {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
        }
    }
}

impl RadioGatun {
    const ROTATION_32: [u32; 19] = [
        0, 1, 3, 6, 10, 15, 21, 28, 4, 13, 23, 2, 14, 27, 9, 24, 8, 25, 11,
    ];
    const ROTATION_64: [u32; 19] = [
        0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 2, 14, 27, 41, 56, 8, 25, 43,
    ];

    // XOR words from the mill into the belt
    fn belt_to_mill_feedforward(belt_words: &mut [u32], mill_words: &[u32]) {
        for i in 0..12 {
            belt_words[i + ((i % 3) * 13)] ^= mill_words[i + 1]
        }
    }

    fn mill(mill_words: &mut [u32]) {
        let mut temp_arr = [0; 19];
        for i in 0..19 {
            let t = (i * 7) % 19;
            let mut k = mill_words[t];
            k = k ^ (mill_words[(t as usize + 1) % 19]) | (!mill_words[(t as usize + 2) % 19]);
            temp_arr[i] = k.rotate_right(Self::ROTATION_32[i])
        }
        for i in 0..19 {
            mill_words[i] = temp_arr[i] ^ temp_arr[(i + 1) % 19] ^ temp_arr[(i + 4) % 19]
        }
    }

    fn rotate_belt(belt_words: &mut [u32]) {
        for i in (0..39).rev() {
            belt_words[i + 1] = belt_words[i]
        }
        for i in 0..3 {
            belt_words[(i + 1) * 13] = belt_words[i * 13]
        }
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
        Self::mill(mill_words);
        Self::rotate_belt(belt_words);
        Self::iota(mill_words);
        Self::belt_to_mill(belt_words, mill_words)
    }
}

impl ClassicHasher for RadioGatun {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        todo!()
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
        hasher.input_format = ByteFormat::Hex;
        hasher.output_format = ByteFormat::Hex;
    }
}
