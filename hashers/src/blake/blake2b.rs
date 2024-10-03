use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

use crate::traits::ClassicHasher;

// https://eprint.iacr.org/2012/351.pdf

#[derive(Debug, Clone)]

pub struct Blake2b {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub key: Vec<u8>,    // optional key, length from 0 to 64 bytes
    pub hash_len: usize, // length of output in bytes, 1 to 64
}

impl Default for Blake2b {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            key: Vec::new(),
            hash_len: 32, // default to 256 bits
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

    pub fn hash_len(mut self, hash_len: usize) -> Self {
        assert!(hash_len > 1 && hash_len <= 64);
        self.hash_len = hash_len;
        self
    }

    pub fn key(mut self, key: Vec<u8>) -> Self {
        assert!(key.len() <= 64);
        self.key = key;
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
    pub fn compress(state: &mut [u64; 8], chunk: &[u64; 16], bytes_taken: u128, last_chunk: bool) {
        // println!("Original Chunk:\n{chunk:016x?}\n");
        // create a working vector
        let mut work = [0_u64; 16];
        for i in 0..8 {
            work[i] = state[i];
            work[i + 8] = Self::IV[i]
        }

        // Mix the bytes taken counter into the working vector
        work[12] ^= bytes_taken as u64; // low bytes
        work[13] ^= (bytes_taken >> 64) as u64; // high bytes

        // invert all bits of the work[14] if the last chunk
        if last_chunk {
            work[14] ^= u64::MAX;
        }

        crate::blake_compress!(&mut work, chunk, [32, 24, 16, 63], 12);

        for i in 0..8 {
            state[i] ^= work[i];
            state[i] ^= work[i + 8];
        }
    }

    fn create_chunk(bytes: &[u8]) -> [u64; 16] {
        let mut k = [0u64; 16];
        for (elem, chunk) in k.iter_mut().zip(bytes.chunks_exact(8)).take(16) {
            *elem = u64::from_le_bytes(chunk.try_into().unwrap());
        }
        k
    }
}

impl ClassicHasher for Blake2b {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        assert!(
            self.hash_len > 1 && self.hash_len <= 64,
            "hash_len cannot be 0 bytes and cannot be greater than 64 bytes"
        );
        assert!(
            self.key.len() <= 64,
            "the length of the key cannot be more than 64 bytes"
        );

        let mut state = Self::IV.clone();

        // Key length and hash length are mixed into the state, this ensures identical inputs don't resemble each other when these inputs are varied
        let mixer: u64 = 0x01010000 ^ ((self.key.len() as u64) << 8) ^ self.hash_len as u64;
        state[0] ^= mixer;

        let mut bytes_taken = 0;
        let mut bytes_remaining = bytes.len();

        // If no key is provided the process of padding out the key and compressing it is skipped
        if self.key.len() > 0 {
            let mut key = self.key.clone();
            while key.len() != 128 {
                key.push(0);
            }
            bytes_taken += 128;
            Self::compress(&mut state, &Self::create_chunk(&key), bytes_taken, false);
        }

        let mut chunks = bytes.chunks_exact(128);

        while bytes_remaining > 128 {
            let chunk = Self::create_chunk(chunks.next().unwrap());
            bytes_taken += 128;
            bytes_remaining -= 128;
            Self::compress(&mut state, &chunk, bytes_taken, false);
        }

        // compress the last chunk, padding with zeroes if it is too short
        let mut last = chunks.remainder().to_vec();
        bytes_taken += last.len() as u128;
        while last.len() != 128 {
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
    empty_hash_len_64, Blake2b::default().hash_len(64), "",
    "786a02f742015903c6c6fd852552d272912f4740e15847618a86e217f71f5419d25e1031afee585313896444934eb04b903a685b1448b755d56f701afe9be2ce";
    abc_hash_len_64, Blake2b::default().hash_len(64), "abc",
    "ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923";
    keyed_hash_len_64,
    Blake2b::default()
        .input(ByteFormat::Hex)
        .hash_len(64)
        .key(ByteFormat::Hex.text_to_bytes("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f").unwrap()),
        "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfe",
    "142709d62e28fcccd0af97fad0f8465b971e82201dc51070faa0372aa43e92484be1c1e73ba10906d5d1853db6a4106e0a7bf9800d373d6dee2d46d62ef2a461";
);
