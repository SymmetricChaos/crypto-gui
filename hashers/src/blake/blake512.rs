use itertools::Itertools;
use utils::byte_formatting::{fill_u64s_be, ByteFormat};

use crate::{errors::HasherError, traits::ClassicHasher};

// https://eprint.iacr.org/2012/351.pdf

// Constants for compression function, beginning digits of pi
const C: [u64; 16] = [
    0x243f6a8885a308d3,
    0x13198a2e03707344,
    0xa4093822299f31d0,
    0x082efa98ec4e6c89,
    0x452821e638d01377,
    0xbe5466cf34e90c6c,
    0xc0ac29b7c97c50dd,
    0x3f84d5b5b5470917,
    0x9216d5d98979fb1b,
    0xd1310ba698dfb5ac,
    0x2ffd72dbd01adfb7,
    0xb8e1afed6a267e96,
    0xba7c9045f12c7f99,
    0x24a19947b3916cf7,
    0x0801f2e2858efc16,
    0x636920d871574e69,
];

#[derive(Debug, Clone)]
pub struct Blake512 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub salt: [u64; 4], // optional salt
    truncated: bool,
}

impl Default for Blake512 {
    fn default() -> Self {
        Blake512::blake512()
    }
}

impl Blake512 {
    // Initialization vector, sqrt of the first eight primes
    const IV_512: [u64; 8] = [
        0x6a09e667f3bcc908,
        0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b,
        0xa54ff53a5f1d36f1,
        0x510e527fade682d1,
        0x9b05688c2b3e6c1f,
        0x1f83d9abfb41bd6b,
        0x5be0cd19137e2179,
    ];

    const IV_384: [u64; 8] = [
        0xcbbb9d5dc1059ed8,
        0x629a292a367cd507,
        0x9159015a3070dd17,
        0x152fecd8f70e5939,
        0x67332667ffc00b31,
        0x8eb44a8768581511,
        0xdb0c2e0d64f98fa7,
        0x47b5481dbefa4fa4,
    ];

    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn salt(mut self, salt: [u64; 4]) -> Self {
        self.salt = salt;
        self
    }

    pub fn salt_from_string(&mut self, text: &str) -> Result<(), HasherError> {
        if text.len() != 64 {
            return Err(HasherError::key(
                "key must be given as exactly 64 hex digits",
            ));
        }
        let v = ByteFormat::Hex
            .text_to_u64_be(text)
            .expect("salt text did not have exactly 64 digits");
        self.salt = v
            .try_into()
            .expect("failed to convert Vec<u64> to [u64; 4]");

        Ok(())
    }

    pub fn blake512() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            salt: [0, 0, 0, 0],
            truncated: false,
        }
    }

    pub fn blake384() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            salt: [0, 0, 0, 0],
            truncated: true,
        }
    }

    // https://decred.org/research/aumasson2010.pdf
    pub fn compress(state: &mut [u64; 8], chunk: &[u64; 16], counter: u128, salt: &[u64; 4]) {
        // create a working vector starting with the current state and then following it with C xored with the salt, then C xored with the counter
        // println!("chunk: {:016x?}\n", chunk);
        let mut work = [0_u64; 16];
        for i in 0..8 {
            work[i] = state[i];
        }
        for i in 0..4 {
            work[i + 8] = C[i] ^ salt[i]
        }
        work[12] = C[4] ^ (counter as u64); // Lower bits
        work[13] = C[5] ^ (counter as u64);
        work[14] = C[6] ^ (counter >> 64) as u64; // Upper bits
        work[15] = C[7] ^ (counter >> 64) as u64;

        crate::blake_compress!(&mut work, chunk, [32, 25, 16, 11], C, 16);

        for i in 0..8 {
            state[i] ^= salt[i % 4] ^ work[i] ^ work[i + 8];
        }
        // println!("intermediate: {:016x?}\n", state);
    }

    fn create_chunk(bytes: &[u8]) -> [u64; 16] {
        let mut k = [0u64; 16];
        fill_u64s_be(&mut k, &bytes);
        k
    }
}

impl ClassicHasher for Blake512 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();

        // Length in bits before padding
        let b_len = (bytes.len().wrapping_mul(8)) as u128;

        // Padding
        // push a byte with a leading 1 to the bytes
        input.push(0x80);
        // push zeros until the length in bits is 888 mod 1024
        // equivalently until the length in bytes is 111 mod 128
        while (input.len() % 128) != 111 {
            input.push(0x00)
        }

        // Final byte before length is 0x01 for BLAKE512 and is 0x00 for BLAKE384
        if self.truncated {
            input.push(0x00);
        } else {
            input.push(0x01);
        }

        // Append length
        for b in b_len.to_be_bytes() {
            input.push(b)
        }

        let mut bytes_remaining = input.len();
        let mut counter = 0;
        let mut state = match self.truncated {
            true => Self::IV_384.clone(),
            false => Self::IV_512.clone(),
        };
        let mut message = input.chunks_exact(128).peekable();

        while bytes_remaining >= 128 {
            let chunk = Self::create_chunk(message.next().unwrap());
            if message.peek().is_none() {
                counter = b_len;
            } else {
                counter += 128;
            }
            bytes_remaining -= 128;
            Self::compress(&mut state, &chunk, counter, &self.salt)
        }

        if self.truncated {
            state
                .iter()
                .take(6)
                .map(|x| x.to_be_bytes())
                .flatten()
                .collect_vec()
        } else {
            state
                .iter()
                .map(|x| x.to_be_bytes())
                .flatten()
                .collect_vec()
        }
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod blake512_tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut hasher = Blake512::default();
        hasher.input_format = ByteFormat::Hex;
        hasher.output_format = ByteFormat::Hex;
        assert_eq!("97961587f6d970faba6d2478045de6d1fabd09b61ae50932054d52bc29d31be4ff9102b9f69e2bbdb83be13d4b9c06091e5fa0b48bd081b634058be0ec49beb3", hasher.hash_bytes_from_string("00").unwrap());
    }
}
