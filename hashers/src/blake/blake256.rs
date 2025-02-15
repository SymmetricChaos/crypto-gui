// use crate::traits::StatefulHasher;
use crate::traits::StatefulHasher;
use itertools::Itertools;
use utils::byte_formatting::fill_u32s_be;

const BLOCK_LEN: usize = 64;

// Constants for compression function, beginning digits of pi
const C: [u32; 16] = [
    0x243f6a88, 0x85a308d3, 0x13198a2e, 0x03707344, 0xa4093822, 0x299f31d0, 0x082efa98, 0xec4e6c89,
    0x452821e6, 0x38d01377, 0xbe5466cf, 0x34e90c6c, 0xc0ac29b7, 0xc97c50dd, 0x3f84d5b5, 0xb5470917,
];

const IV_256: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

const IV_224: [u32; 8] = [
    0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939, 0xffc00b31, 0x68581511, 0x64f98fa7, 0xbefa4fa4,
];

// https://decred.org/research/aumasson2010.pdf
pub fn compress(state: &mut [u32; 8], chunk: &[u32; 16], counter: u64, salt: &[u32; 4]) {
    // create a working vector starting with the current state and then following it with the IV xored with the salt, then the IV xored with the counter
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

    crate::blake_compress!(&mut work, chunk, [16, 12, 8, 7], C, 14);

    for i in 0..8 {
        state[i] ^= salt[i % 4] ^ work[i] ^ work[i + 8];
    }
}

fn create_chunk(bytes: &[u8]) -> [u32; 16] {
    let mut k = [0u32; 16];
    fill_u32s_be(&mut k, &bytes);
    k
}

#[derive(Debug, Clone)]
pub struct Blake224 {
    state: [u32; 8],
    salt: [u32; 4],
    bits_taken: u64,
    buffer: Vec<u8>,
}

impl Blake224 {
    pub fn init() -> Self {
        Self {
            state: IV_224,
            salt: [0; 4],
            bits_taken: 0,
            buffer: Vec::with_capacity(BLOCK_LEN),
        }
    }

    pub fn init_mac(salt: [u32; 4]) -> Self {
        Self {
            state: IV_224,
            salt,
            bits_taken: 0,
            buffer: Vec::with_capacity(BLOCK_LEN),
        }
    }
}

impl StatefulHasher for Blake224 {
    fn update(&mut self, mut bytes: &[u8]) {
        while !bytes.is_empty() {
            if self.buffer.len() == BLOCK_LEN {
                let c = create_chunk(&self.buffer);
                self.bits_taken += 512;
                compress(&mut self.state, &c, self.bits_taken, &self.salt);
                self.buffer.clear();
            }
            crate::take_bytes!(self.buffer, bytes, BLOCK_LEN);
        }
    }

    fn finalize(mut self) -> Vec<u8> {
        // Padding
        let total_bits = self.bits_taken + (self.buffer.len() * 8) as u64;

        if self.buffer.len() == 55 {
            self.buffer.push(0x81);
        } else {
            self.buffer.push(0x80);
            while self.buffer.len() % 64 != 55 {
                self.buffer.push(0x00);
            }
            self.buffer.push(0x00); // different from BLAKE256
        }

        self.buffer.extend(total_bits.to_be_bytes());

        // There could be either one or two blocks to finalize
        if self.buffer.len() > 64 {
            let c = create_chunk(&self.buffer[0..64]);
            compress(&mut self.state, &c, total_bits, &self.salt);
            let c = create_chunk(&self.buffer[64..128]);
            compress(&mut self.state, &c, 0, &self.salt); // in the two final block case the last block has its counter set to zero
        } else {
            let c = create_chunk(&self.buffer);
            compress(&mut self.state, &c, total_bits, &self.salt);
        }

        self.state
            .iter()
            .take(7)
            .map(|x| x.to_be_bytes())
            .flatten()
            .collect_vec()
    }

    crate::stateful_hash_helpers!();
}

#[derive(Debug, Clone)]
pub struct Blake256 {
    state: [u32; 8],
    salt: [u32; 4],
    bits_taken: u64,
    buffer: Vec<u8>,
}

impl Blake256 {
    pub fn init() -> Self {
        let hasher = Self {
            state: IV_256,
            salt: [0; 4],
            bits_taken: 0,
            buffer: Vec::with_capacity(BLOCK_LEN),
        };

        hasher
    }

    pub fn init_mac(salt: [u32; 4]) -> Self {
        let hasher = Self {
            state: IV_256,
            salt,
            bits_taken: 0,
            buffer: Vec::with_capacity(BLOCK_LEN),
        };

        hasher
    }
}

impl StatefulHasher for Blake256 {
    fn update(&mut self, mut bytes: &[u8]) {
        while !bytes.is_empty() {
            if self.buffer.len() == BLOCK_LEN {
                let c = create_chunk(&self.buffer);
                self.bits_taken += 512;
                compress(&mut self.state, &c, self.bits_taken, &self.salt);
                self.buffer.clear();
            }
            crate::take_bytes!(self.buffer, bytes, BLOCK_LEN);
        }
    }

    fn finalize(mut self) -> Vec<u8> {
        // Padding
        let total_bits = self.bits_taken + (self.buffer.len() * 8) as u64;

        if self.buffer.len() == 55 {
            self.buffer.push(0x81);
        } else {
            self.buffer.push(0x80);
            while self.buffer.len() % 64 != 55 {
                self.buffer.push(0x00);
            }
            self.buffer.push(0x01);
        }

        self.buffer.extend(total_bits.to_be_bytes());

        // There could be either one or two blocks to finalize
        if self.buffer.len() > 64 {
            let c = create_chunk(&self.buffer[0..64]);
            compress(&mut self.state, &c, total_bits, &self.salt);
            let c = create_chunk(&self.buffer[64..128]);
            compress(&mut self.state, &c, 0, &self.salt); // in the two final block case the last block has its counter set to zero
        } else {
            let c = create_chunk(&self.buffer);
            compress(&mut self.state, &c, total_bits, &self.salt);
        }

        self.state
            .iter()
            .map(|x| x.to_be_bytes())
            .flatten()
            .collect_vec()
    }

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    test_256_one_byte, Blake256::init(),
    &[0x00],
    "0ce8d4ef4dd7cd8d62dfded9d4edb0a774ae6a41929a74da23109e8f11139c87";

    test_512_pangram, Blake256::init(),
    &[0x00; 72],
    "d419bad32d504fb7d44d460c42c5593fe544fa4c135dec31e21bd9abdcc22d41";
);
