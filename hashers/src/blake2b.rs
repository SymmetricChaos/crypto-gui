use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

use crate::{errors::HasherError, traits::ClassicHasher};

// https://eprint.iacr.org/2012/351.pdf

pub struct Blake2b {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: Vec<u8>,    // optional key, length from 0 to 64 bytes
    pub hash_len: usize, // length of output in bytes, 1 to 64
}

impl Default for Blake2b {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            key: Vec::new(),
            hash_len: 16,
        }
    }
}

impl Blake2b {
    // Initialization vector, sqrt of the first eight primes
    const IV: [u64; 8] = [
        0x6a09e667f3bcc908,
        0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b,
        0xa54ff53a5f1d36f1,
        0x510e527fade682d1,
        0x9b05688c2b3e6c1f,
        0x1f83d9abfb41bd6b,
        0x5be0cd19137e2179,
    ];

    // Message permutation schedule
    const SIGMA: [[usize; 16]; 12] = [
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
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], // the 11th and 12th rounds reuse the permutations from the 1st and 2nd rounds
        [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
    ];

    pub fn mix(v: &mut [u64; 16], a: usize, b: usize, c: usize, d: usize, x: u64, y: u64) {
        v[a] = v[a].wrapping_add(v[b]).wrapping_add(x);
        v[d] = (v[d] ^ v[a]).rotate_right(32);

        v[c] = v[c].wrapping_add(v[d]);
        v[b] = (v[b] ^ v[c]).rotate_right(24);

        v[a] = v[a].wrapping_add(v[b]).wrapping_add(y);
        v[d] = (v[d] ^ v[a]).rotate_right(16);

        v[c] = v[c].wrapping_add(v[d]);
        v[b] = (v[b] ^ v[c]).rotate_right(63);
    }

    pub fn compress(state: &mut [u64; 8], chunk: &[u64; 8], bytes_taken: u128, last_chunk: bool) {
        // create a working vector
        let mut work = [0_u64; 16];
        for i in 0..8 {
            work[i] = state[i];
            work[i + 8] = Self::IV[i]
        }

        // Mix the bytes taken counter into the working vector
        work[12] = (bytes_taken >> 64) as u64;
        work[13] = bytes_taken as u64;

        // invert all bits of the work[14] if the last chunk
        if last_chunk {
            work[14] ^= u64::MAX;
        }

        for i in 0..11 {
            let s = Self::SIGMA[i];

            Self::mix(&mut work, 0, 4, 8, 12, chunk[s[0]], chunk[s[1]]);
            Self::mix(&mut work, 1, 5, 9, 13, chunk[s[2]], chunk[s[3]]);
            Self::mix(&mut work, 2, 6, 10, 14, chunk[s[4]], chunk[s[5]]);
            Self::mix(&mut work, 3, 7, 11, 15, chunk[s[6]], chunk[s[7]]);

            Self::mix(&mut work, 0, 5, 10, 15, chunk[s[8]], chunk[s[9]]);
            Self::mix(&mut work, 1, 6, 11, 12, chunk[s[10]], chunk[s[11]]);
            Self::mix(&mut work, 2, 7, 8, 13, chunk[s[12]], chunk[s[13]]);
            Self::mix(&mut work, 3, 4, 9, 14, chunk[s[14]], chunk[s[15]]);
        }

        for i in 0..8 {
            state[i] ^= work[i];
            state[i] ^= work[i + 8];
        }
    }

    fn create_chunk(bytes: &[u8]) -> [u64; 8] {
        let mut k = [0u64; 8];
        for (elem, chunk) in k.iter_mut().zip(bytes.chunks_exact(8)).take(16) {
            *elem = u64::from_be_bytes(chunk.try_into().unwrap());
        }
        k
    }
}

impl ClassicHasher for Blake2b {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        if self.hash_len > 64 {
            panic!("hash_len cannot be greater than 64 as there are only 64 bytes of state")
        }

        let mut state = Self::IV.clone();
        // Key length and hash length are mixed into the state, this ensures identical inputs don't resemble each other when these inputs are varied
        let mixer: u64 = 0x01010000 ^ ((self.key.len() as u64) << 8) ^ self.hash_len as u64;
        state[0] ^= mixer;

        let mut bytes_taken = 0;
        let mut bytes_remaining = bytes.len();

        let mut key = self.key.clone();
        while key.len() != 128 {
            key.push(0);
        }

        Self::compress(&mut state, &Self::create_chunk(&key), bytes_taken, false);

        let mut chunks = bytes.chunks_exact(128);

        while bytes_remaining > 128 {
            let chunk = Self::create_chunk(chunks.next().unwrap());
            Self::compress(&mut state, &chunk, bytes_taken, false);
            bytes_taken += 128;
            bytes_remaining -= 128;
        }

        let mut last = chunks.remainder().to_vec();
        while last.len() != 128 {
            last.push(0);
        }
        Self::compress(&mut state, &Self::create_chunk(&last), bytes_taken, true);

        // compress the last chunk, padding with zeroes if it is too short

        state
            .iter()
            .map(|x| x.to_be_bytes())
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
mod blake2b_tests {
    use super::*;

    #[test]
    fn test() {}
}
