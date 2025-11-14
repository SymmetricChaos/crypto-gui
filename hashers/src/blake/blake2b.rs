use crate::traits::StatefulHasher;
use itertools::Itertools;

// https://eprint.iacr.org/2012/351.pdf

const BLOCK_LEN: usize = 128;

// https://datatracker.ietf.org/doc/html/rfc7693.html#appendix-A
pub fn compress(state: &mut [u64; 8], chunk: &[u64; 16], bytes_taken: u128, last_chunk: bool) {
    // create a working vector
    let mut work = [0_u64; 16];
    for i in 0..8 {
        work[i] = state[i];
        work[i + 8] = IV[i]
    }

    // Mix the bytes from the counter into the working vector
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

#[derive(Debug, Clone)]
pub struct Blake2b {
    state: [u64; 8],
    hash_len: u64, // length of output in bytes, 1 to 64
    bytes_taken: u128,
    buffer: Vec<u8>,
}

impl Blake2b {
    pub fn init<T: AsRef<[u8]>>(key: T, hash_len: u64) -> Self {
        assert!(key.as_ref().len() <= 64);
        assert!(hash_len > 1 && hash_len <= 64);
        let mut hasher = Self {
            state: IV,
            hash_len,
            bytes_taken: 0,
            buffer: Vec::with_capacity(BLOCK_LEN),
        };

        // The key length and hash length are mixed into the state
        let mixer: u64 = 0x01010000 ^ ((key.as_ref().len() as u64) << 8) ^ hash_len as u64;
        hasher.state[0] ^= mixer;

        // Pad and include the key
        if key.as_ref().len() > 0 {
            let mut key = key.as_ref().to_vec();
            while key.len() != 128 {
                key.push(0);
            }
            hasher.bytes_taken += 128;
            compress(
                &mut hasher.state,
                &create_chunk(&key),
                hasher.bytes_taken,
                false,
            );
        }

        hasher
    }

    pub fn init_hash_256() -> Self {
        Self::init(&[], 32)
    }

    pub fn init_hash_384() -> Self {
        Self::init(&[], 48)
    }

    pub fn init_hash_512() -> Self {
        Self::init(&[], 64)
    }

    pub fn init_hash(hash_len: u64) -> Self {
        Self::init(&[], hash_len)
    }

    pub fn init_mac_256<T: AsRef<[u8]>>(key: T) -> Self {
        Self::init(key, 32)
    }

    pub fn init_mac_384<T: AsRef<[u8]>>(key: T) -> Self {
        Self::init(key, 48)
    }

    pub fn init_mac_512<T: AsRef<[u8]>>(key: T) -> Self {
        Self::init(key, 64)
    }

    pub fn init_mac_var<T: AsRef<[u8]>>(key: T, hash_len: u64) -> Self {
        Self::init(key, hash_len)
    }

    pub fn hash_len(&self) -> u64 {
        self.hash_len
    }

    pub fn hash_256(bytes: &[u8]) -> Vec<u8> {
        Self::init_hash_256().hash(bytes)
    }

    pub fn hash_384(bytes: &[u8]) -> Vec<u8> {
        Self::init_hash_384().hash(bytes)
    }

    pub fn hash_512(bytes: &[u8]) -> Vec<u8> {
        Self::init_hash_512().hash(bytes)
    }
}

impl StatefulHasher for Blake2b {
    fn update(&mut self, mut bytes: &[u8]) {
        crate::compression_routine!(self.buffer, bytes, BLOCK_LEN, {
            let c = create_chunk(&self.buffer);
            self.bytes_taken += 128;
            compress(&mut self.state, &c, self.bytes_taken, false);
        });
    }

    fn finalize(mut self) -> Vec<u8> {
        self.bytes_taken += self.buffer.len() as u128;
        while self.buffer.len() < BLOCK_LEN {
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
mod tests {
    use super::*;

    const KEY: &[u8] = &[
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
        0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c,
        0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b,
        0x3c, 0x3d, 0x3e, 0x3f,
    ];

    const MSG: [u8; 255] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
        0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c,
        0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b,
        0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a,
        0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59,
        0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68,
        0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77,
        0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86,
        0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95,
        0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4,
        0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3,
        0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2,
        0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1,
        0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0,
        0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef,
        0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe,
    ];

    crate::stateful_hash_tests!(
        empty, Blake2b::init_hash_512(), &[], "786a02f742015903c6c6fd852552d272912f4740e15847618a86e217f71f5419d25e1031afee585313896444934eb04b903a685b1448b755d56f701afe9be2ce";
        abc, Blake2b::init_hash_512(), &[0x61, 0x62, 0x63], "ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923";
        with_key, Blake2b::init(KEY, 64), &MSG, "142709d62e28fcccd0af97fad0f8465b971e82201dc51070faa0372aa43e92484be1c1e73ba10906d5d1853db6a4106e0a7bf9800d373d6dee2d46d62ef2a461";
    );
}
