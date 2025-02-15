use crate::traits::StatefulHasher;
use itertools::Itertools;

// https://eprint.iacr.org/2012/351.pdf

const BLOCK_LEN: usize = 64;

// https://datatracker.ietf.org/doc/html/rfc7693.html#appendix-A
pub fn compress(state: &mut [u32; 8], chunk: &[u32; 16], bytes_taken: u64, last_chunk: bool) {
    // create a working vector
    let mut work = [0_u32; 16];
    for i in 0..8 {
        work[i] = state[i];
        work[i + 8] = IV[i]
    }

    // Mix the bytes from the counter into the working vector
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
pub struct Blake2s {
    state: [u32; 8],
    hash_len: u32, // length of output in bytes, 1 to 32
    bytes_taken: u64,
    buffer: Vec<u8>,
}

impl Blake2s {
    pub fn init<T: AsRef<[u8]>>(key: T, hash_len: u32) -> Self {
        assert!(key.as_ref().len() <= 32);
        assert!(hash_len > 1 && hash_len <= 32);
        let mut hasher = Self {
            state: IV,
            hash_len,
            bytes_taken: 0,
            buffer: Vec::with_capacity(BLOCK_LEN),
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
    pub fn init_hash_128() -> Self {
        Self::init(&[], 32)
    }

    pub fn init_hash_256() -> Self {
        Self::init(&[], 32)
    }

    pub fn init_hash_var(hash_len: u32) -> Self {
        Self::init(&[], hash_len)
    }

    pub fn init_mac_128<T: AsRef<[u8]>>(key: T) -> Self {
        Self::init(key, 32)
    }

    pub fn init_mac_256<T: AsRef<[u8]>>(key: T) -> Self {
        Self::init(key, 32)
    }

    pub fn init_mac_var<T: AsRef<[u8]>>(key: T, hash_len: u32) -> Self {
        Self::init(key, hash_len)
    }

    pub fn hash_len(&self) -> u32 {
        self.hash_len
    }

    pub fn state(&self) -> &[u32; 8] {
        &self.state
    }

    pub fn state_bytes(&self) -> Vec<u8> {
        self.state
            .iter()
            .map(|x| x.to_le_bytes())
            .flatten()
            .take(self.hash_len as usize)
            .collect_vec()
    }

    pub fn hash_128(bytes: &[u8]) -> Vec<u8> {
        Self::init_hash_128().update_and_finalize(bytes)
    }

    pub fn hash_256(bytes: &[u8]) -> Vec<u8> {
        Self::init_hash_256().update_and_finalize(bytes)
    }
}

impl StatefulHasher for Blake2s {
    fn update(&mut self, mut bytes: &[u8]) {
        while !bytes.is_empty() {
            if self.buffer.len() == BLOCK_LEN {
                let c = create_chunk(&self.buffer);
                self.bytes_taken += 64;
                compress(&mut self.state, &c, self.bytes_taken, false);
                self.buffer.clear();
            }
            crate::take_bytes!(self.buffer, bytes, BLOCK_LEN);
        }
    }

    fn finalize(mut self) -> Vec<u8> {
        self.bytes_taken += self.buffer.len() as u64;
        println!("{}", self.bytes_taken);
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

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    empty, Blake2s::init_hash_256(), &[], "69217a3079908094e11121d042354a7c1f55b6482ca1a51e1b250dfd1ed0eef9";
    digits, Blake2s::init_hash_256(), &[0, 1, 2, 3, 4, 5, 6, 7], "c7e887b546623635e93e0495598f1726821996c2377705b93a1f636f872bfa2d";
    with_key, Blake2s::init([0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f], 32),&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], "19ba234f0a4f38637d1839f9d9f76ad91c8522307143c97d5f93f69274cec9a7";
);

crate::incremental_hash_tests!(

    digits, Blake2s::init_hash_256(),
    &[
        &[0, 1, 2, 3],
        &[4, 5, 6, 7],
    ],
    "c7e887b546623635e93e0495598f1726821996c2377705b93a1f636f872bfa2d";
);
