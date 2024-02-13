use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

use crate::{errors::HasherError, traits::ClassicHasher};

// https://eprint.iacr.org/2012/351.pdf

pub struct Blake256 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub salt: [u32; 4],  // optional salt
    pub hash_len: usize, // length of output in bytes, 1 to 64
}

impl Default for Blake256 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            salt: [0, 0, 0, 0],
            hash_len: 32, // default to 256 bits
        }
    }
}

impl Blake256 {
    // Initialization vector, sqrt of the first eight primes
    const IV: [u32; 8] = [
        0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB,
        0x5BE0CD19,
    ];

    // const IV_224: [u32; 8] = [
    //     0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939, 0xffc00b31, 0x68581511, 0x64f98fa7,
    //     0xbefa4fa4,
    // ];

    // Constants for compression function, beginning digits of pi
    const C: [u32; 16] = [
        0x243f6a88, 0x85a308d3, 0x13198a2e, 0x03707344, 0xa4093822, 0x299f31d0, 0x082efa98,
        0xec4e6c89, 0x452821e6, 0x38d01377, 0xbe5466cf, 0x34e90c6c, 0xc0ac29b7, 0xc97c50dd,
        0x3f84d5b5, 0xb5470917,
    ];

    // Message permutation schedule
    const SIGMA: [[usize; 16]; 10] = [
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
        [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
        [7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
        [9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
        [2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
        [12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11],
        [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10],
        [6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5],
        [10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0],
    ];

    pub fn mix(v: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize, x: u32, y: u32) {
        v[a] = v[a].wrapping_add(v[b]).wrapping_add(x);
        v[d] = (v[d] ^ v[a]).rotate_right(16);

        v[c] = v[c].wrapping_add(v[d]);
        v[b] = (v[b] ^ v[c]).rotate_right(12);

        v[a] = v[a].wrapping_add(v[b]).wrapping_add(y);
        v[d] = (v[d] ^ v[a]).rotate_right(8);

        v[c] = v[c].wrapping_add(v[d]);
        v[b] = (v[b] ^ v[c]).rotate_right(7);
    }

    // https://decred.org/research/aumasson2010.pdf
    pub fn compress(state: &mut [u32; 8], chunk: &[u32; 16], counter: u64, salt: &[u32; 4]) {
        // create a working vector starting with the current state and then following it with the IV xored with the salt, then the IV xored with the counter
        let mut work = [0_u32; 16];
        for i in 0..8 {
            work[i] = state[i];
        }
        for i in 0..4 {
            work[i + 8] = Self::IV[i] ^ salt[i]
        }
        work[12] = Self::IV[12] ^ (counter >> 32) as u32; // Upper bits
        work[13] = Self::IV[13] ^ (counter >> 32) as u32;
        work[14] = Self::IV[14] ^ (counter as u32); // Lower bits
        work[15] = Self::IV[15] ^ (counter as u32);

        for i in 0..14 {
            let s = Self::SIGMA[i % 10];

            // Apply the mixing function eight times, xoring the constants with the chunks of message
            for j in 0..4 {
                let x = chunk[s[2 * j]] ^ Self::C[s[2 * j + 1]];
                let y = chunk[s[2 * j + 1]] ^ Self::C[s[2 * j]];
                Self::mix(&mut work, j, j + 4, j + 8, j + 12, x, y);
            }

            for j in 0..4 {
                let x = chunk[s[2 * j]] ^ Self::C[s[2 * j + 1]];
                let y = chunk[s[2 * j + 1]] ^ Self::C[s[2 * j]];
                Self::mix(&mut work, j, j + 5, j + 10, j + 15, x, y);
            }
        }

        for i in 0..8 {
            state[i] ^= salt[i % 4] ^ work[i] ^ work[i + 8];
        }
    }

    fn create_chunk(bytes: &[u8]) -> [u32; 16] {
        let mut k = [0u32; 16];
        for (elem, chunk) in k.iter_mut().zip(bytes.chunks_exact(8)).take(16) {
            *elem = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        k
    }
}

impl ClassicHasher for Blake256 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();

        // Length in bits before padding
        let b_len = (input.len().wrapping_mul(8)) as u64;

        // Step 1.Padding
        // push a byte with a leading 1 to the bytes
        input.push(0x80);
        // push zeros until the length in bits is 440 mod 512
        // equivalently until the length in bytes is 55 mod 64
        while (input.len() % 64) != 55 {
            input.push(0)
        }

        // Final byte before length is 0x01
        input.push(0x01);

        // Append length
        for b in b_len.to_be_bytes() {
            input.push(b)
        }

        let mut counter: u64 = 0;
        let mut state = Self::IV.clone();

        let mut chunks = input.chunks_exact(128);

        state
            .iter()
            .map(|x| x.to_le_bytes())
            .flatten()
            .take(self.hash_len)
            .collect_vec()
    }

    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| HasherError::general("byte format error"))?;
        let out = self.hash(&mut bytes);
        Ok(self.output_format.bytes_to_text(&out))
    }
}

#[cfg(test)]
mod blake256_tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut hasher = Blake256::default();
        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;
        hasher.hash_len = 64;
        assert_eq!("", hasher.hash_bytes_from_string("").unwrap());
    }

    #[test]
    fn test_abc() {
        let mut hasher = Blake256::default();
        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;

        hasher.hash_len = 64;
        assert_eq!("", hasher.hash_bytes_from_string("abc").unwrap());
    }

    #[test]
    fn test_keyed() {
        let mut hasher = Blake256::default();
        hasher.input_format = ByteFormat::Hex;
        hasher.output_format = ByteFormat::Hex;
        hasher.hash_len = 64;
        hasher.key = ByteFormat::Hex.text_to_bytes("").unwrap();
        assert_eq!("", hasher.hash_bytes_from_string("").unwrap());
    }
}
