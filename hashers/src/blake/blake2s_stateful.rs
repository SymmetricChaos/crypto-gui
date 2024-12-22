use itertools::Itertools;

use crate::traits::StatefulHasher;

// https://eprint.iacr.org/2012/351.pdf

// https://datatracker.ietf.org/doc/html/rfc7693.html#appendix-A
pub fn compress(state: &mut [u32; 8], chunk: &[u32; 16], bytes_taken: u64, last_chunk: bool) {
    // create a working vector
    let mut work = [0_u32; 16];
    for i in 0..8 {
        work[i] = state[i];
        work[i + 8] = IV[i]
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

// Initialization vector, sqrt of the first eight primes
const IV: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

#[derive(Debug, Clone)]
pub struct Blake2bStateful {
    state: [u32; 8],
    hash_len: u32, // length of output in bytes, 1 to 64
    bytes_taken: u64,
    buffer: Vec<u8>,
}

impl Blake2bStateful {
    pub fn init<T: AsRef<[u8]>>(key: T, hash_len: u32) -> Self {
        assert!(key.as_ref().len() <= 32);
        assert!(hash_len > 1 && hash_len <= 32);
        let mut hasher = Self {
            state: IV,
            hash_len,
            bytes_taken: 0,
            buffer: Vec::new(),
        };

        // The key length and hash length are mixed into the state
        let mixer: u32 = 0x01010000 ^ ((key.as_ref().len() as u32) << 8) ^ hash_len as u32;
        hasher.state[0] ^= mixer;

        // Pad and include the key
        if key.as_ref().len() > 0 {
            let mut key = key.as_ref().to_vec();
            while key.len() != 64 {
                key.push(0);
            }
            hasher.bytes_taken += 64;
            compress(
                &mut hasher.state,
                &create_chunk(&key),
                hasher.bytes_taken,
                false,
            );
        }

        hasher
    }

    pub fn hash_len(&self) -> u32 {
        self.hash_len
    }

    pub fn state(&self) -> &[u32; 8] {
        &self.state
    }
}

impl StatefulHasher for Blake2bStateful {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
        let chunks = bytes.chunks_exact(128);
        let last = chunks.remainder().to_vec();

        for chunk in chunks {
            let c = create_chunk(chunk);
            self.bytes_taken += 128;
            compress(&mut self.state, &c, self.bytes_taken, false);
        }

        self.buffer = last;
    }

    fn finalize(&mut self) -> Vec<u8> {
        self.bytes_taken += self.buffer.len() as u64;
        while self.buffer.len() < 128 {
            self.buffer.push(0);
        }
        compress(
            &mut self.state,
            &create_chunk(&self.buffer),
            self.bytes_taken,
            true,
        );
        self.state
            .iter()
            .map(|x| x.to_le_bytes())
            .flatten()
            .take(self.hash_len as usize)
            .collect_vec()
    }
}

#[cfg(test)]
mod blake2b_stateful_tests {

    use utils::byte_formatting::ByteFormat;

    use super::*;

    #[test]
    fn test_empty() {
        let mut hasher = Blake2bStateful::init(&[], 32);
        assert_eq!(
            hasher.finalize(),
            ByteFormat::Hex
                .text_to_bytes("69217a3079908094e11121d042354a7c1f55b6482ca1a51e1b250dfd1ed0eef9")
                .unwrap()
        );
    }

    #[test]
    fn test_digits() {
        let mut hasher = Blake2bStateful::init(&[], 32);
        hasher.update(&[0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(
            hasher.finalize(),
            ByteFormat::Hex
                .text_to_bytes("c7e887b546623635e93e0495598f1726821996c2377705b93a1f636f872bfa2d")
                .unwrap()
        );
    }

    #[test]
    fn test_with_key() {
        let mut hasher = Blake2bStateful::init(
            ByteFormat::Hex
                .text_to_bytes("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f")
                .unwrap(),
            32,
        );
        hasher.update(
            &ByteFormat::Hex
                .text_to_bytes("000102030405060708090a0b0c0d0e0f")
                .unwrap(),
        );
        assert_eq!(
            hasher.finalize(),
            ByteFormat::Hex
                .text_to_bytes("19ba234f0a4f38637d1839f9d9f76ad91c8522307143c97d5f93f69274cec9a7")
                .unwrap()
        );
    }
}
