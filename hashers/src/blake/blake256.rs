use itertools::Itertools;
use utils::byte_formatting::ByteFormat;

use crate::{errors::HasherError, traits::ClassicHasher};

use super::SIGMA;

// https://eprint.iacr.org/2012/351.pdf

// Constants for compression function, beginning digits of pi
const C: [u32; 16] = [
    0x243f6a88, 0x85a308d3, 0x13198a2e, 0x03707344, 0xa4093822, 0x299f31d0, 0x082efa98, 0xec4e6c89,
    0x452821e6, 0x38d01377, 0xbe5466cf, 0x34e90c6c, 0xc0ac29b7, 0xc97c50dd, 0x3f84d5b5, 0xb5470917,
];

#[derive(Debug, Clone)]
pub struct Blake256 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub salt: [u32; 4], // optional salt
    truncated: bool,
}

impl Default for Blake256 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            salt: [0, 0, 0, 0],
            truncated: false,
        }
    }
}

impl Blake256 {
    // Initialization vector, sqrt of the first eight primes
    const IV_256: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    const IV_224: [u32; 8] = [
        0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939, 0xffc00b31, 0x68581511, 0x64f98fa7,
        0xbefa4fa4,
    ];

    pub fn salt_from_string(&mut self, text: &str) -> Result<(), HasherError> {
        if text.len() != 32 {
            return Err(HasherError::key(
                "key must be given as exactly 32 hex digits",
            ));
        }
        let v = ByteFormat::Hex
            .text_to_u32(text)
            .expect("salt text did not have exactly 32 digits");
        self.salt = v
            .try_into()
            .expect("failed to convert Vec<u32> to [u32; 4]");

        Ok(())
    }

    pub fn blake256() -> Self {
        Self::default()
    }

    pub fn blake224() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            salt: [0, 0, 0, 0],
            truncated: true,
        }
    }

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
        // println!("chunk: {:08x?}\n", chunk);
        let mut work = [0_u32; 16];
        for i in 0..8 {
            work[i] = state[i];
        }
        for i in 0..4 {
            work[i + 8] = C[i] ^ salt[i]
        }
        work[12] = C[4] ^ (counter as u32); // Lower bits
        work[13] = C[5] ^ (counter as u32);
        work[14] = C[6] ^ (counter >> 32) as u32; // Upper bits
        work[15] = C[7] ^ (counter >> 32) as u32;

        // At this point the working vector is correct, I have triple checked
        // println!("work: {:08x?}\n", work);
        for i in 0..14 {
            let s = SIGMA[i % 10];

            let a = [0, 1, 2, 3, 0, 1, 2, 3];
            let b = [4, 5, 6, 7, 5, 6, 7, 4];
            let c = [8, 9, 10, 11, 10, 11, 8, 9];
            let d = [12, 13, 14, 15, 15, 12, 13, 14];

            // Apply the mixing function eight times, xoring the constants with the chunks of message
            for j in 0..8 {
                let x = chunk[s[2 * j]] ^ C[s[2 * j + 1]];
                let y = chunk[s[2 * j + 1]] ^ C[s[2 * j]];
                Self::mix(&mut work, a[j], b[j], c[j], d[j], x, y);
            }

            // println!("work {}:\n{:08x?}\n", i + 1, work);
        }
        for i in 0..8 {
            state[i] ^= salt[i % 4] ^ work[i] ^ work[i + 8];
        }
        // println!("intermediate: {:08x?}\n", state);
    }

    fn create_chunk(bytes: &[u8]) -> [u32; 16] {
        let mut k = [0u32; 16];
        for (elem, chunk) in k.iter_mut().zip(bytes.chunks_exact(4)).take(16) {
            *elem = u32::from_be_bytes(chunk.try_into().unwrap());
        }
        k
    }
}

impl ClassicHasher for Blake256 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();

        // Length in bits before padding
        let b_len = (bytes.len().wrapping_mul(8)) as u64;

        // Padding
        // push a byte with a leading 1 to the bytes
        input.push(0x80);
        // push zeros until the length in bits is 440 mod 512
        // equivalently until the length in bytes is 55 mod 64
        while (input.len() % 64) != 55 {
            input.push(0x00)
        }

        // Final byte before length is 0x01 for BLAKE256 and is 0x00 for BLAKE224
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
            true => Self::IV_224.clone(),
            false => Self::IV_256.clone(),
        };
        let mut message = input.chunks_exact(64).peekable();

        while bytes_remaining >= 64 {
            let chunk = Self::create_chunk(message.next().unwrap());
            if message.peek().is_none() {
                counter = b_len;
            } else {
                counter += 512;
            }
            bytes_remaining -= 64;
            Self::compress(&mut state, &chunk, counter, &self.salt)
        }

        if self.truncated {
            state
                .iter()
                .take(7)
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
mod blake256_tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut hasher = Blake256::default();
        hasher.input_format = ByteFormat::Hex;
        hasher.output_format = ByteFormat::Hex;

        assert_eq!(
            "0ce8d4ef4dd7cd8d62dfded9d4edb0a774ae6a41929a74da23109e8f11139c87",
            hasher.hash_bytes_from_string("00").unwrap()
        );
        assert_eq!(
            "d419bad32d504fb7d44d460c42c5593fe544fa4c135dec31e21bd9abdcc22d41",
            hasher.hash_bytes_from_string("000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap()
        );
    }
}
