use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

use crate::{errors::HasherError, traits::ClassicHasher};

use super::SIGMA;

// https://eprint.iacr.org/2012/351.pdf

#[derive(Debug, Clone)]

pub struct Blake2s {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: Vec<u8>,    // optional key, length from 0 to 32 bytes
    pub hash_len: usize, // length of output in bytes, 1 to 32
}

impl Default for Blake2s {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            key: Vec::new(),
            hash_len: 16, // default to 128 bits
        }
    }
}

impl Blake2s {
    // Initialization vector, sqrt of the first eight primes
    const IV: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
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

    // https://datatracker.ietf.org/doc/html/rfc7693.html#appendix-A
    pub fn compress(state: &mut [u32; 8], chunk: &[u32; 16], bytes_taken: u64, last_chunk: bool) {
        // println!("Original Chunk:\n{chunk:016x?}\n");
        // create a working vector
        let mut work = [0_u32; 16];
        for i in 0..8 {
            work[i] = state[i];
            work[i + 8] = Self::IV[i]
        }

        // Mix the bytes taken counter into the working vector
        work[12] ^= bytes_taken as u32; // low bytes
        work[13] ^= (bytes_taken >> 32) as u32; // high bytes

        // invert all bits of the work[14] if the last chunk
        if last_chunk {
            work[14] ^= u32::MAX;
        }
        // println!("Working Vector Before Compression:\n{work:016x?}\n");
        for i in 0..10 {
            let s = SIGMA[i];

            Self::mix(&mut work, 0, 4, 8, 12, chunk[s[0]], chunk[s[1]]);
            Self::mix(&mut work, 1, 5, 9, 13, chunk[s[2]], chunk[s[3]]);
            Self::mix(&mut work, 2, 6, 10, 14, chunk[s[4]], chunk[s[5]]);
            Self::mix(&mut work, 3, 7, 11, 15, chunk[s[6]], chunk[s[7]]);

            Self::mix(&mut work, 0, 5, 10, 15, chunk[s[8]], chunk[s[9]]);
            Self::mix(&mut work, 1, 6, 11, 12, chunk[s[10]], chunk[s[11]]);
            Self::mix(&mut work, 2, 7, 8, 13, chunk[s[12]], chunk[s[13]]);
            Self::mix(&mut work, 3, 4, 9, 14, chunk[s[14]], chunk[s[15]]);
            // println!("Working Vector at [{i}]:\n{work:016x?}\n");
        }

        for i in 0..8 {
            state[i] ^= work[i];
            state[i] ^= work[i + 8];
        }
    }

    fn create_chunk(bytes: &[u8]) -> [u32; 16] {
        let mut k = [0u32; 16];
        for (elem, chunk) in k.iter_mut().zip(bytes.chunks_exact(4)).take(16) {
            *elem = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        k
    }
}

impl ClassicHasher for Blake2s {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        if self.hash_len > 32 {
            panic!("hash_len cannot be greater than 32 as there are only 32 bytes of state")
        }

        if self.hash_len == 0 {
            panic!("hash_len cannot be zero, obviously")
        }

        if self.key.len() > 32 {
            panic!("the length of the key cannot be more than 32 bytes")
        }

        let mut state = Self::IV.clone();

        // Key length and hash length are mixed into the state, this ensures identical inputs don't resemble each other when these inputs are varied
        let mixer: u32 = 0x01010000 ^ ((self.key.len() as u32) << 8) ^ self.hash_len as u32;
        state[0] ^= mixer;

        let mut bytes_taken = 0;
        let mut bytes_remaining = bytes.len();

        // If no key is provided the process of padding out the key and compressing it is skipped
        if self.key.len() > 0 {
            let mut key = self.key.clone();
            while key.len() != 64 {
                key.push(0);
            }
            bytes_taken += 64;
            Self::compress(&mut state, &Self::create_chunk(&key), bytes_taken, false);
        }

        let mut chunks = bytes.chunks_exact(64);

        while bytes_remaining > 64 {
            let chunk = Self::create_chunk(chunks.next().unwrap());
            bytes_taken += 64;
            bytes_remaining -= 64;
            Self::compress(&mut state, &chunk, bytes_taken, false);
        }

        // compress the last chunk, padding with zeroes if it is too short
        let mut last = chunks.remainder().to_vec();
        bytes_taken += last.len() as u64;
        while last.len() != 64 {
            last.push(0);
        }

        Self::compress(&mut state, &Self::create_chunk(&last), bytes_taken, true);

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
        Ok(self.output_format.byte_slice_to_text(&out))
    }
}

#[cfg(test)]
mod blake2s_tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut hasher = Blake2s::default();
        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;
        hasher.hash_len = 32;
        assert_eq!(
            "69217a3079908094e11121d042354a7c1f55b6482ca1a51e1b250dfd1ed0eef9",
            hasher.hash_bytes_from_string("").unwrap()
        );
    }
}
