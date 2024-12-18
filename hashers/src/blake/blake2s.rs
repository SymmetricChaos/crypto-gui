use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

use crate::traits::ClassicHasher;

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
            input_format: ByteFormat::Utf8,
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

    pub fn hash_len(mut self, hash_len: usize) -> Self {
        assert!(hash_len > 1 && hash_len <= 32);
        self.hash_len = hash_len;
        self
    }

    pub fn key<T: AsRef<[u8]>>(mut self, key: T) -> Self {
        assert!(key.as_ref().len() <= 32);
        self.key = key.as_ref().to_vec();
        self
    }

    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
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

        crate::blake_compress!(&mut work, chunk, [16, 12, 8, 7], 10);

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
        assert!(
            self.hash_len > 1 && self.hash_len <= 32,
            "hash_len cannot be 0 bytes and cannot be greater than 32 bytes"
        );
        assert!(
            self.key.len() <= 32,
            "the length of the key cannot be more than 32 bytes"
        );

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

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    empty_hash_len_32, Blake2s::default().hash_len(32), "",
    "69217a3079908094e11121d042354a7c1f55b6482ca1a51e1b250dfd1ed0eef9";
    hash_8_len_32, Blake2s::default().hash_len(32).input(ByteFormat::Hex), "0001020304050607",
    "c7e887b546623635e93e0495598f1726821996c2377705b93a1f636f872bfa2d";
    keyed_hash_len_32,Blake2s::default()
        .input(ByteFormat::Hex)
        .hash_len(32)
        .key(ByteFormat::Hex.text_to_bytes("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f").unwrap()),
        "000102030405060708090a0b0c0d0e0f",
    "19ba234f0a4f38637d1839f9d9f76ad91c8522307143c97d5f93f69274cec9a7";
);
