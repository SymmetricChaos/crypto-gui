use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

use crate::{errors::HasherError, traits::ClassicHasher};

use super::SIGMA;

// https://github.com/BLAKE3-team/BLAKE3
// https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.pdf

pub struct Blake3 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: [u8; 32], // optional 256-bit key
    pub keyed_hash: bool,
    // pub derive_key: bool, // ignoring this for now
}

impl Default for Blake3 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            key: [0; 32],
            keyed_hash: false,
            // derive_key: false, // ignoring this for now
        }
    }
}

impl Blake3 {
    // Initialization vector, sqrt of the first eight primes
    const IV: [u32; 8] = [
        0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB,
        0x5BE0CD19,
    ];

    pub fn set_keyed_hash_flag(&mut self) {}

    pub fn set_key_derivation_flag(&mut self) {}

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

    pub fn create_initialization_vector(&self) -> [u32; 8] {
        if self.keyed_hash {
            let mut k = [0u32; 8];
            for (elem, chunk) in k.iter_mut().zip(self.key.chunks_exact(4)).take(8) {
                *elem = u32::from_le_bytes(chunk.try_into().unwrap());
            }
            k
        } else {
            Self::IV
        }
    }

    // https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.pdf
    // Blocks of 64 bytes (512 bits) are compressed as a sequence of u32s
    pub fn compress_block(
        &self,
        state: &mut [u32; 8],
        block: &[u32; 16],
        initialization_vector: &[u32; 8],
        counter: u64,
        byte_length: u32,
        flags: u32,
    ) {
        // println!("Original Chunk:\n{chunk:016x?}\n");
        // create a working vector
        let mut work = [0_u32; 16];
        for i in 0..8 {
            work[i] = state[i];
        }

        for i in 0..4 {
            work[i + 8] = initialization_vector[i]
        }

        // Mix the bytes taken counter into the working vector
        work[12] ^= counter as u32; // low bytes
        work[13] ^= (counter >> 32) as u32; // high bytes

        // work[14] ^= bytes in the chunk
        work[15] ^= flags;

        // println!("Working Vector Before Compression:\n{work:016x?}\n");
        for i in 0..7 {
            let s = SIGMA[i];

            Self::mix(&mut work, 0, 4, 8, 12, block[s[0]], block[s[1]]);
            Self::mix(&mut work, 1, 5, 9, 13, block[s[2]], block[s[3]]);
            Self::mix(&mut work, 2, 6, 10, 14, block[s[4]], block[s[5]]);
            Self::mix(&mut work, 3, 7, 11, 15, block[s[6]], block[s[7]]);

            Self::mix(&mut work, 0, 5, 10, 15, block[s[8]], block[s[9]]);
            Self::mix(&mut work, 1, 6, 11, 12, block[s[10]], block[s[11]]);
            Self::mix(&mut work, 2, 7, 8, 13, block[s[12]], block[s[13]]);
            Self::mix(&mut work, 3, 4, 9, 14, block[s[14]], block[s[15]]);
            // println!("Working Vector at [{i}]:\n{work:016x?}\n");
        }

        for i in 0..8 {
            state[i] ^= work[i];
            state[i] ^= work[i + 8];
        }
    }

    // Up to 1024 bytes
    // fn create_chunk(bytes: &[u8]) -> [u32; 16] {
    //     let mut k = [0u32; 16];
    //     for (elem, chunk) in k.iter_mut().zip(bytes.chunks_exact(4)).take(16) {
    //         *elem = u32::from_le_bytes(chunk.try_into().unwrap());
    //     }
    //     k
    // }

    // Up to 64 bytes
    fn create_block(bytes: &[u8]) -> [u32; 16] {
        let mut k = [0u32; 16];
        for (elem, chunk) in k.iter_mut().zip(bytes.chunks_exact(4)).take(16) {
            *elem = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        k
    }
}

impl ClassicHasher for Blake3 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut state = Self::IV.clone();

        let initialization_vector = self.create_initialization_vector();

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

        // Divide the input into chunks of 1024 bytes
        let mut chunks = bytes.chunks_exact(1024);

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
        let mut hasher = Blake3::default();
        hasher.input_format = ByteFormat::Utf8;
        hasher.output_format = ByteFormat::Hex;
        hasher.hash_len = 32;
        assert_eq!(
            "69217a3079908094e11121d042354a7c1f55b6482ca1a51e1b250dfd1ed0eef9",
            hasher.hash_bytes_from_string("").unwrap()
        );
    }
}
